// Generated macro for none (function)
macro_rules! Depcrate_matchers_none_matchernone {
() => {
// Module: crate::matchers::none_matcher
// Provides: {"none"}
// Dependencies: {}
# [doc = " Matches an `Option` containing `None`."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " verify_that!(None::<()>, none())?;   // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail() -> Result<()> {"] # [doc = " verify_that!(Some(\"Some value\"), none())?;  // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " # should_fail().unwrap_err();"] # [doc = " ```"] pub fn none () -> NoneMatcher { NoneMatcher }
};
}
