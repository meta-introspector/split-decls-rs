// Generated macro for into_iter_deep_call (function)
macro_rules! Depcrate_useless_conversioninto_iter_deep_call {
() => {
// Module: crate::useless_conversion
// Provides: {"into_iter_deep_call"}
// Dependencies: {}
# [doc = " Same as [`into_iter_call`], but tries to look for the innermost `.into_iter()` call, e.g.:"] # [doc = " `foo.into_iter().into_iter()`"] # [doc = "  ^^^  we want this expression"] fn into_iter_deep_call < 'hir > (cx : & LateContext < '_ > , mut expr : & 'hir Expr < 'hir >) -> (& 'hir Expr < 'hir > , usize) { let mut depth = 0 ; while let Some (recv) = into_iter_call (cx , expr) { expr = recv ; depth += 1 ; } (expr , depth) }
};
}
