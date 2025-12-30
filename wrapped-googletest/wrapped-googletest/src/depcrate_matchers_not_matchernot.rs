// Generated macro for not (function)
macro_rules! Depcrate_matchers_not_matchernot {
() => {
// Module: crate::matchers::not_matcher
// Provides: {"not"}
// Dependencies: {}
# [doc = " Matches the actual value exactly when the inner matcher does _not_ match."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " verify_that!(0, not(eq(1)))?; // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail() -> Result<()> {"] # [doc = " verify_that!(0, not(eq(0)))?; // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " # should_fail().unwrap_err();"] # [doc = " ```"] pub fn not < InnerMatcherT > (inner : InnerMatcherT) -> NotMatcher < InnerMatcherT > { NotMatcher { inner } }
};
}
