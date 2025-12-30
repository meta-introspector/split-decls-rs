// Generated macro for impl_6664 (impl)
macro_rules! Depcrate_methods_swap_with_temporaryimpl_6664 {
() => {
// Module: crate::methods::swap_with_temporary
// Provides: {"impl_6664"}
// Dependencies: {}
impl < 'tcx > ArgKind < 'tcx > { # [doc = " Build a new `ArgKind` from `arg`. There must be no false positive when returning a"] # [doc = " `ArgKind::RefMutToTemp` variant, as this may cause a spurious lint to be emitted."] fn new (cx : & LateContext < 'tcx > , arg : & 'tcx Expr < 'tcx >) -> Self { if let ExprKind :: AddrOf (BorrowKind :: Ref , _ , target) = arg . kind && let adjustments = cx . typeck_results () . expr_adjustments (arg) && adjustments . first () . is_some_and (| adj | matches ! (adj . kind , Adjust :: Deref (None))) && adjustments . last () . is_some_and (| adj | matches ! (adj . kind , Adjust :: Borrow (_))) { let extra_derefs = adjustments [1 .. adjustments . len () - 1] . iter () . filter (| adj | matches ! (adj . kind , Adjust :: Deref (_))) . count () ; if target . is_syntactic_place_expr () || extra_derefs > 0 { if arg . span . from_expansion () { ArgKind :: RefMutToPlaceAsMacro (arg , extra_derefs) } else { ArgKind :: RefMutToPlace (target , extra_derefs) } } else { ArgKind :: RefMutToTemp (target) } } else { ArgKind :: Expr (arg) } } }
};
}
