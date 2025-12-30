// Generated macro for is_consistent_order (function)
macro_rules! Depcrate_inconsistent_struct_constructoris_consistent_order {
() => {
// Module: crate::inconsistent_struct_constructor
// Provides: {"is_consistent_order"}
// Dependencies: {}
fn is_consistent_order < 'tcx > (fields : & 'tcx [hir :: ExprField < 'tcx >] , def_order_map : & FxHashMap < Symbol , usize >) -> bool { let mut cur_idx = usize :: MIN ; for f in fields { let next_idx = def_order_map [& f . ident . name] ; if cur_idx > next_idx { return false ; } cur_idx = next_idx ; } true }
};
}
