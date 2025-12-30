// Generated macro for mut_borrows_in_expr (function)
macro_rules! Depcrate_operators_assign_op_patternmut_borrows_in_expr {
() => {
// Module: crate::operators::assign_op_pattern
// Provides: {"mut_borrows_in_expr"}
// Dependencies: {}
fn mut_borrows_in_expr (cx : & LateContext < '_ > , e : & hir :: Expr < '_ >) -> HirIdSet { struct S (HirIdSet) ; impl Delegate < '_ > for S { fn borrow (& mut self , place : & PlaceWithHirId < '_ > , _ : HirId , kind : BorrowKind) { if matches ! (kind , BorrowKind :: Mutable) { self . 0 . insert (match place . place . base { PlaceBase :: Local (id) => id , PlaceBase :: Upvar (id) => id . var_path . hir_id , _ => return , }) ; } } fn consume (& mut self , _ : & PlaceWithHirId < '_ > , _ : HirId) { } fn use_cloned (& mut self , _ : & PlaceWithHirId < '_ > , _ : HirId) { } fn mutate (& mut self , _ : & PlaceWithHirId < '_ > , _ : HirId) { } fn fake_read (& mut self , _ : & PlaceWithHirId < '_ > , _ : FakeReadCause , _ : HirId) { } fn copy (& mut self , _ : & PlaceWithHirId < '_ > , _ : HirId) { } } let mut s = S (HirIdSet :: default ()) ; let v = ExprUseVisitor :: for_clippy (cx , e . hir_id . owner . def_id , & mut s) ; v . consume_expr (e) . into_ok () ; s . 0 }
};
}
