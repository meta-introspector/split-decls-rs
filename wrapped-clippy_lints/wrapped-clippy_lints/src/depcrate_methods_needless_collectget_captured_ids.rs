// Generated macro for get_captured_ids (function)
macro_rules! Depcrate_methods_needless_collectget_captured_ids {
() => {
// Module: crate::methods::needless_collect
// Provides: {"get_captured_ids"}
// Dependencies: {}
fn get_captured_ids (cx : & LateContext < '_ > , ty : Ty < '_ >) -> HirIdSet { fn get_captured_ids_recursive (cx : & LateContext < '_ > , ty : Ty < '_ > , set : & mut HirIdSet) { match ty . kind () { ty :: Adt (_ , generics) => { for generic in * generics { if let GenericArgKind :: Type (ty) = generic . kind () { get_captured_ids_recursive (cx , ty , set) ; } } } , ty :: Closure (def_id , _) => { let closure_hir_node = cx . tcx . hir_get_if_local (* def_id) . unwrap () ; if let Node :: Expr (closure_expr) = closure_hir_node { can_move_expr_to_closure (cx , closure_expr) . unwrap () . into_iter () . for_each (| (hir_id , capture_kind) | { if matches ! (capture_kind , CaptureKind :: Ref (Mutability :: Mut)) { set . insert (hir_id) ; } }) ; } } , _ => () , } } let mut set = HirIdSet :: default () ; get_captured_ids_recursive (cx , ty , & mut set) ; set }
};
}
