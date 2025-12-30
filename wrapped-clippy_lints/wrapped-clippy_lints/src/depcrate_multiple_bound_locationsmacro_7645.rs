// Generated macro for macro_7645 (macro)
macro_rules! Depcrate_multiple_bound_locationsmacro_7645 {
() => {
// Module: crate::multiple_bound_locations
// Provides: {"macro_7645"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Check if a generic is defined both in the bound predicate and in the `where` clause."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can be confusing for developers when seeing bounds for a generic in multiple places."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn ty<F: std::fmt::Debug>(a: F)"] # [doc = " where"] # [doc = "     F: Sized,"] # [doc = " {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn ty<F>(a: F)"] # [doc = " where"] # [doc = "     F: Sized + std::fmt::Debug,"] # [doc = " {}"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub MULTIPLE_BOUND_LOCATIONS , suspicious , "defining generic bounds in multiple locations" }
};
}
