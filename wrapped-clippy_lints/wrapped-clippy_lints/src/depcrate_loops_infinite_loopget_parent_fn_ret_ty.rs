// Generated macro for get_parent_fn_ret_ty (function)
macro_rules! Depcrate_loops_infinite_loopget_parent_fn_ret_ty {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"get_parent_fn_ret_ty"}
// Dependencies: {}
fn get_parent_fn_ret_ty < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ >) -> Option < FnRetTy < 'tcx > > { for (_ , parent_node) in cx . tcx . hir_parent_iter (expr . hir_id) { match parent_node { Node :: Expr (Expr { kind : ExprKind :: Closure (Closure { kind : ClosureKind :: Coroutine (_) , .. }) , .. }) => () , Node :: Item (hir :: Item { kind : hir :: ItemKind :: Fn { sig : FnSig { decl , .. } , .. } , .. }) | Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Fn (FnSig { decl , .. } , _) , .. }) | Node :: ImplItem (hir :: ImplItem { kind : hir :: ImplItemKind :: Fn (FnSig { decl , .. } , _) , .. }) | Node :: Expr (Expr { kind : ExprKind :: Closure (Closure { fn_decl : decl , .. }) , .. }) => return Some (decl . output) , _ => () , } } None }
};
}
