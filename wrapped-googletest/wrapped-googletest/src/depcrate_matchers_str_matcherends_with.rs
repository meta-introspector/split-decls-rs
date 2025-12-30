// Generated macro for ends_with (function)
macro_rules! Depcrate_matchers_str_matcherends_with {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"ends_with"}
// Dependencies: {}
# [doc = " Matches a string which ends with the given suffix."] # [doc = ""] # [doc = " Both the actual value and the expected suffix may be either a `String` or"] # [doc = " a string reference."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass_1() -> Result<()> {"] # [doc = " verify_that!(\"Some value\", ends_with(\"value\"))?;  // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_1() -> Result<()> {"] # [doc = " verify_that!(\"Some value\", ends_with(\"other value\"))?;   // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_fail_2() -> Result<()> {"] # [doc = " verify_that!(\"Some value\", ends_with(\"Some\"))?;  // Fails"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # fn should_pass_2() -> Result<()> {"] # [doc = " verify_that!(\"Some value\".to_string(), ends_with(\"value\"))?;   // Passes"] # [doc = " verify_that!(\"Some value\", ends_with(\"value\".to_string()))?;   // Passes"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass_1().unwrap();"] # [doc = " # should_fail_1().unwrap_err();"] # [doc = " # should_fail_2().unwrap_err();"] # [doc = " # should_pass_2().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " See the [`StrMatcherConfigurator`] extension trait for more options on how"] # [doc = " the string is matched."] pub fn ends_with < T > (expected : T) -> StrMatcher < T > { StrMatcher { configuration : Configuration { mode : MatchMode :: EndsWith , .. Default :: default () } , expected , } }
};
}
