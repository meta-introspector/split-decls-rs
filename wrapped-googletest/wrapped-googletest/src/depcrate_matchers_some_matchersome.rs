// Generated macro for some (function)
macro_rules! Depcrate_matchers_some_matchersome {
() => {
// Module: crate::matchers::some_matcher
// Provides: {"some"}
// Dependencies: {}
# [doc = " Matches an `Option` containing a value matched by `inner`."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " verify_that!(Some(\"Some value\"), some(eq(\"Some value\")))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_1() -> Result<()> {"] # [doc = " verify_that!(None::<&str>, some(eq(\"Some value\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_2() -> Result<()> {"] # [doc = " verify_that!(Some(\"Some value\"), some(eq(\"Some other value\")))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " # should_fail_1().unwrap_err();"] # [doc = " # should_fail_2().unwrap_err();"] # [doc = " ```"] pub fn some < Inner > (inner : Inner) -> SomeMatcher < Inner > { SomeMatcher { inner } }
};
}
