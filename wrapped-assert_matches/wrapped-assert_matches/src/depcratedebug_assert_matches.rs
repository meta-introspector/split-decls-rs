// Generated macro for debug_assert_matches (macro)
macro_rules! Depcratedebug_assert_matches {
() => {
// Module: crate
// Provides: {"debug_assert_matches"}
// Dependencies: {}
# [doc = " Asserts that an expression matches a given pattern."] # [doc = ""] # [doc = " Unlike [`assert_matches!`], `debug_assert_matches!` statements are only enabled"] # [doc = " in non-optimized builds by default. An optimized build will omit all"] # [doc = " `debug_assert_matches!` statements unless `-C debug-assertions` is passed"] # [doc = " to the compiler."] # [doc = ""] # [doc = " See the macro [`assert_matches!`] documentation for more information."] # [doc = ""] # [doc = " [`assert_matches!`]: macro.assert_matches.html"] # [macro_export (local_inner_macros)] macro_rules ! debug_assert_matches { ($ ($ tt : tt) *) => { { if _assert_matches_cfg ! (debug_assertions) { assert_matches ! ($ ($ tt) *) ; } } } }
};
}
