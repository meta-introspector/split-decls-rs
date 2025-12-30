// Generated macro for __result_of_ref (macro)
macro_rules! Depcrate_matchers_result_of_matcher__result_of_ref {
() => {
// Module: crate::matchers::result_of_matcher
// Provides: {"__result_of_ref"}
// Dependencies: {}
# [doc = " Matches a value where the reference to the result of `callable` applied to"] # [doc = " the value matches the inner matcher."] # [doc = ""] # [doc = " The `callable` will be called twice, so make sure it is pure."] # [doc = " ```"] # [doc = " use googletest::prelude::*;"] # [doc = " fn should_pass_1() -> googletest::Result<()> {"] # [doc = "    verify_that!(\"hello\", result_of_ref!(|s: &str| s.to_uppercase(), eq(\"HELLO\")))?; // Passes"] # [doc = "    Ok(())"] # [doc = " }"] # [doc = ""] # [doc = " fn should_pass_2() -> googletest::Result<()> {"] # [doc = "    verify_that!(100, result_of_ref!(|value| value + 1, eq(&101)))?; // Passes"] # [doc = "    Ok(())"] # [doc = " }"] # [doc = ""] # [doc = " fn should_fail() -> googletest::Result<()> {"] # [doc = "    verify_that!(\"world\", result_of_ref!(|s: &str| s.to_uppercase(), eq(\"HELLO\")))?; // Passes"] # [doc = "    Ok(())"] # [doc = " }"] # [doc = " should_pass_1().unwrap();"] # [doc = " should_pass_2().unwrap();"] # [doc = " should_fail().unwrap_err();"] # [doc = " ```"] # [macro_export] macro_rules ! __result_of_ref { ($ function : expr , $ matcher : expr) => { { $ crate :: matchers :: __internal_unstable_do_not_depend_on_these :: result_of_ref ($ function , $ matcher , stringify ! ($ function) ,) } } ; }
};
}
