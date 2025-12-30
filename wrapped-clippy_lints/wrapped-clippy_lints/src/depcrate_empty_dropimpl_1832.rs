// Generated macro for impl_1832 (impl)
macro_rules! Depcrate_empty_dropimpl_1832 {
() => {
// Module: crate::empty_drop
// Provides: {"impl_1832"}
// Dependencies: {}
impl LateLintPass < '_ > for EmptyDrop { fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , items : [child] , .. }) = item . kind && of_trait . trait_ref . trait_def_id () == cx . tcx . lang_items () . drop_trait () && let impl_item_hir = child . hir_id () && let Node :: ImplItem (impl_item) = cx . tcx . hir_node (impl_item_hir) && let ImplItemKind :: Fn (_ , b) = & impl_item . kind && let Body { value : func_expr , .. } = cx . tcx . hir_body (* b) && let func_expr = peel_blocks (func_expr) && let ExprKind :: Block (block , _) = func_expr . kind && block . stmts . is_empty () && block . expr . is_none () { span_lint_and_then (cx , EMPTY_DROP , item . span , "empty drop implementation" , | diag | { diag . span_suggestion_hidden (item . span , "try removing this impl" , String :: new () , Applicability :: MaybeIncorrect ,) ; }) ; } } }
};
}
