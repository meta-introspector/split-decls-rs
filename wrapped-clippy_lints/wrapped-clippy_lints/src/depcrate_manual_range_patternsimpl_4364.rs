// Generated macro for impl_4364 (impl)
macro_rules! Depcrate_manual_range_patternsimpl_4364 {
() => {
// Module: crate::manual_range_patterns
// Provides: {"impl_4364"}
// Dependencies: {}
impl Num { fn new (expr : & PatExpr < '_ >) -> Option < Self > { Some (Self { val : expr_as_i128 (expr) ? , span : expr . span , }) } fn dummy (val : i128) -> Self { Self { val , span : DUMMY_SP } } fn min (self , other : Self) -> Self { if self . val < other . val { self } else { other } } }
};
}
