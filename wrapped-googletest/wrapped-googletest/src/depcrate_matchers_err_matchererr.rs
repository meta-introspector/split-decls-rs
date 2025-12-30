// Generated macro for err (function)
macro_rules! Depcrate_matchers_err_matchererr {
() => {
// Module: crate::matchers::err_matcher
// Provides: {"err"}
// Dependencies: {}
# [doc = " Matches a `Result` containing `Err` with a value matched by `inner`."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> googletest::Result<()> {"] # [doc = " verify_that!(Err::<(), _>(\"Some error\"), err(eq(\"Some error\")))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_1() -> googletest::Result<()> {"] # [doc = " verify_that!(Ok::<_, &str>(\"A value\"), err(eq(\"A value\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_2() -> googletest::Result<()> {"] # [doc = " verify_that!(Err::<(), _>(\"Some error\"), err(eq(\"Some other error\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " # should_fail_1().unwrap_err();"] # [doc = " # should_fail_2().unwrap_err();"] # [doc = " ```"] pub fn err < Inner > (inner : Inner) -> ErrMatcher < Inner > { ErrMatcher { inner } }
};
}
