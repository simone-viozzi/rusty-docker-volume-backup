/*
    for integration tests we will use colima to create a local docker environment
    and test in it, then tear it down at the end of the tests.

    TODO: implement the test framework that run colima start, run the tests, then colima stop
*/

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(1, 1);
    }
}
