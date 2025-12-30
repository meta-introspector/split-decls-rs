// Generated macro for ok (function)
macro_rules! Depcrate_matchers_ok_matcherok {
() => {
// Module: crate::matchers::ok_matcher
// Provides: {"ok"}
// Dependencies: {}
# [doc = " Matches a `Result` containing `Ok` with a value matched by `inner`."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> googletest::Result<()> {"] # [doc = " verify_that!(Ok::<_, ()>(\"Some value\"), ok(eq(\"Some value\")))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_1() -> googletest::Result<()> {"] # [doc = " verify_that!(Err::<&str, _>(\"An error\"), ok(eq(\"An error\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_2() -> googletest::Result<()> {"] # [doc = " verify_that!(Ok::<_, ()>(\"Some value\"), ok(eq(\"Some other value\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " # should_fail_1().unwrap_err();"] # [doc = " # should_fail_2().unwrap_err();"] # [doc = " ```"] pub fn ok < InnerMatcherT > (inner : InnerMatcherT) -> OkMatcher < InnerMatcherT > { OkMatcher { inner } }
};
}
