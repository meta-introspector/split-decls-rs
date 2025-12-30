// Generated macro for contains (function)
macro_rules! Depcrate_matchers_contains_matchercontains {
() => {
// Module: crate::matchers::contains_matcher
// Provides: {"contains"}
// Dependencies: {}
# [doc = " Matches an [`IntoIterator`] type whose elements contain a value matched by"] # [doc = " `inner`."] # [doc = ""] # [doc = " By default, this matches a container with any number of elements matched"] # [doc = " by `inner`. Use the method [`ContainsMatcher::times`] to constrain the"] # [doc = " matched containers to a specific number of matching elements."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " verify_that!([\"Some value\"], contains(eq(\"Some value\")))?;  // Passes"] # [doc = " verify_that!(vec![\"Some value\"], contains(eq(&\"Some value\")))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_1() -> Result<()> {"] # [doc = " verify_that!([] as [&String; 0], contains(eq(\"Some value\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_2() -> Result<()> {"] # [doc = " verify_that!([\"Some value\"], contains(eq(\"Some other value\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " # should_fail_1().unwrap_err();"] # [doc = " # should_fail_2().unwrap_err();"] # [doc = " ```"] pub fn contains < InnerMatcherT > (inner : InnerMatcherT) -> ContainsMatcher < InnerMatcherT > { ContainsMatcher { inner , count : None } }
};
}
