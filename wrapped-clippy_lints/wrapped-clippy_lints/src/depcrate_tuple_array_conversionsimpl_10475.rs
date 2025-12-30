// Generated macro for impl_10475 (impl)
macro_rules! Depcrate_tuple_array_conversionsimpl_10475 {
() => {
// Module: crate::tuple_array_conversions
// Provides: {"impl_10475"}
// Dependencies: {}
impl LateLintPass < '_ > for TupleArrayConversions { fn check_expr < 'tcx > (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if expr . span . in_external_macro (cx . sess () . source_map ()) || ! self . msrv . meets (cx , msrvs :: TUPLE_ARRAY_CONVERSIONS) { return ; } match expr . kind { ExprKind :: Array (elements) if (1 ..= 12) . contains (& elements . len ()) => check_array (cx , expr , elements) , ExprKind :: Tup (elements) if (1 ..= 12) . contains (& elements . len ()) => check_tuple (cx , expr , elements) , _ => { } , } } }
};
}
