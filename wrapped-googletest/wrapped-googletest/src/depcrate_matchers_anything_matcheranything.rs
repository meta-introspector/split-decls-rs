// Generated macro for anything (function)
macro_rules! Depcrate_matchers_anything_matcheranything {
() => {
// Module: crate::matchers::anything_matcher
// Provides: {"anything"}
// Dependencies: {}
# [doc = " Matches anything. This matcher always succeeds."] # [doc = ""] # [doc = " This is useful to check if `actual` matches the specific structure (like"] # [doc = " `Some(...)`)  but without caring about the internal value."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " let option = Some(\"Some value\");"] # [doc = " verify_that!(option, some(anything()))?;"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " ```"] pub fn anything () -> Anything { Anything }
};
}
