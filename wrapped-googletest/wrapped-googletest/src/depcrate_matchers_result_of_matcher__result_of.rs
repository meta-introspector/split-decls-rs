// Generated macro for __result_of (macro)
macro_rules! Depcrate_matchers_result_of_matcher__result_of {
() => {
// Module: crate::matchers::result_of_matcher
// Provides: {"__result_of"}
// Dependencies: {}
# [doc = " Matches a value where the result of `callable` applied to the value matches"] # [doc = " the inner matcher."] # [doc = ""] # [doc = " The `callable` will be called twice, so make sure it is pure."] # [doc = " ```"] # [doc = " use googletest::prelude::*;"] # [doc = " fn should_pass() -> googletest::Result<()> {"] # [doc = "    verify_that!(100, result_of!(|value| value + 1, eq(101)))?; // Passes"] # [doc = "    Ok(())"] # [doc = " }"] # [doc = ""] # [doc = " fn should_fail() -> googletest::Result<()> {"] # [doc = "    verify_that!(100, result_of!(|value| value * 2, eq(100)))?; // Fails"] # [doc = "    Ok(())"] # [doc = " }"] # [doc = " should_pass().unwrap();"] # [doc = " should_fail().unwrap_err();"] # [doc = " ```"] # [macro_export] macro_rules ! __result_of { ($ function : expr , $ matcher : expr) => { { $ crate :: matchers :: __internal_unstable_do_not_depend_on_these :: result_of ($ function , $ matcher , stringify ! ($ function) ,) } } ; }
};
}
