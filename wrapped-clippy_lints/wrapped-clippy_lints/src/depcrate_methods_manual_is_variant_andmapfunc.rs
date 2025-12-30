// Generated macro for MapFunc (enum)
macro_rules! Depcrate_methods_manual_is_variant_andMapFunc {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"MapFunc"}
// Dependencies: {}
# [doc = " Represents the argument of the `.map()` function, as a closure or as a path"] # [doc = " in case η-reduction is used."] enum MapFunc < 'hir > { Closure (& 'hir Closure < 'hir >) , Path (& 'hir Expr < 'hir >) , }
};
}
