// Generated macro for impl_7915 (impl)
macro_rules! Depcrate_needless_late_initimpl_7915 {
() => {
// Module: crate::needless_late_init
// Provides: {"impl_7915"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NeedlessLateInit { fn check_local (& mut self , cx : & LateContext < 'tcx > , local : & 'tcx LetStmt < 'tcx >) { let mut parents = cx . tcx . hir_parent_iter (local . hir_id) ; if let LetStmt { init : None , pat : Pat { kind : PatKind :: Binding (BindingMode :: NONE , binding_id , _ , None) , .. } , source : LocalSource :: Normal , .. } = local && let Some ((_ , Node :: Stmt (local_stmt))) = parents . next () && let Some ((_ , Node :: Block (block))) = parents . next () { check (cx , local , local_stmt , block , * binding_id) ; } } }
};
}
