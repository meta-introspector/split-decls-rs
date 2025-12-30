// Generated macro for assert_display_snapshot (macro)
macro_rules! Depcrate_macrosassert_display_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_display_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`Display`](std::fmt::Display) snapshot."] # [doc = ""] # [doc = " This is now deprecated, replaced by the more generic [`assert_snapshot!`](crate::assert_snapshot!)"] # [macro_export] # [deprecated = "use assert_snapshot!() instead"] macro_rules ! assert_display_snapshot { ($ ($ arg : tt) *) => { $ crate :: assert_snapshot ! ($ ($ arg) *) } ; }
};
}
