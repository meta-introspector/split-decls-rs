// Generated macro for impl_10843 (impl)
macro_rules! Depcrate_unit_typesimpl_10843 {
() => {
// Module: crate::unit_types
// Provides: {"impl_10843"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnitTypes { fn check_local (& mut self , cx : & LateContext < 'tcx > , local : & 'tcx LetStmt < 'tcx >) { let_unit_value :: check (cx , & self . format_args , local) ; } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { unit_cmp :: check (cx , expr) ; unit_arg :: check (cx , expr) ; } }
};
}
