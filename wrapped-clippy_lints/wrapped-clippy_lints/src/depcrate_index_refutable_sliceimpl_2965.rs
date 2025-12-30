// Generated macro for impl_2965 (impl)
macro_rules! Depcrate_index_refutable_sliceimpl_2965 {
() => {
// Module: crate::index_refutable_slice
// Provides: {"impl_2965"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for SliceIndexLintingVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let Some (local_id) = expr . res_local_id () { let Self { cx , ref mut slice_lint_info , max_suggested_slice , } = * self ; if let Some (use_info) = slice_lint_info . get_mut (& local_id) && let parent_id = cx . tcx . parent_hir_id (expr . hir_id) && let hir :: Node :: Expr (parent_expr) = cx . tcx . hir_node (parent_id) && let hir :: ExprKind :: Index (_ , index_expr , _) = parent_expr . kind && let Some (Constant :: Int (index_value)) = ConstEvalCtxt :: new (cx) . eval (index_expr) && let Ok (index_value) = index_value . try_into () && index_value < max_suggested_slice && let hir :: Node :: Expr (maybe_addrof_expr) = cx . tcx . parent_hir_node (parent_id) && let hir :: ExprKind :: AddrOf (_kind , hir :: Mutability :: Not , _inner_expr) = maybe_addrof_expr . kind { use_info . index_use . push ((index_value , cx . tcx . hir_span (parent_expr . hir_id))) ; return ; } self . slice_lint_info . swap_remove (& local_id) ; } intravisit :: walk_expr (self , expr) ; } }
};
}
