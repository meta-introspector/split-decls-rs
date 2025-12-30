// Generated macro for Result (type)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A `Result` whose `Err` variant indicates a test failure."] # [doc = ""] # [doc = " The assertions [`verify_that!`][crate::verify_that],"] # [doc = " [`verify_pred!`][crate::verify_pred], and [`fail!`][crate::fail] evaluate"] # [doc = " to `Result<()>`. A test function may return `Result<()>` in combination with"] # [doc = " those macros to abort immediately on assertion failure."] # [doc = ""] # [doc = " This can be used with subroutines which may cause the test to fatally fail"] # [doc = " and which return some value needed by the caller. For example:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " fn load_file_content_as_string() -> Result<String> {"] # [doc = "     let file_stream = load_file().err_to_test_failure()?;"] # [doc = "     Ok(file_stream.to_string())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The `Err` variant contains a [`TestAssertionFailure`] which carries the data"] # [doc = " of the (fatal) assertion failure which generated this result. Non-fatal"] # [doc = " assertion failures, which log the failure and report the test as having"] # [doc = " failed but allow it to continue running, are not encoded in this type."] pub type Result < T , E = TestAssertionFailure > = std :: result :: Result < T , E > ;
};
}
