// Generated macro for TestCaseError (enum)
macro_rules! Depcrate_test_runner_errorsTestCaseError {
() => {
// Module: crate::test_runner::errors
// Provides: {"TestCaseError"}
// Dependencies: {}
# [doc = " Errors which can be returned from test cases to indicate non-successful"] # [doc = " completion."] # [doc = ""] # [doc = " Note that in spite of the name, `TestCaseError` is currently *not* an"] # [doc = " instance of `Error`, since otherwise `impl<E : Error> From<E>` could not be"] # [doc = " provided."] # [doc = ""] # [doc = " Any `Error` can be converted to a `TestCaseError`, which places"] # [doc = " `Error::display()` into the `Fail` case."] # [derive (Debug , Clone)] pub enum TestCaseError { # [doc = " The input was not valid for the test case. This does not count as a"] # [doc = " test failure (nor a success); rather, it simply signals to generate"] # [doc = " a new input and try again."] Reject (Reason) , # [doc = " The code under test failed the test."] Fail (Reason) , }
};
}
