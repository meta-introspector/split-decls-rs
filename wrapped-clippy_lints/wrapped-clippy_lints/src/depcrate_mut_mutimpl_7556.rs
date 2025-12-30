// Generated macro for impl_7556 (impl)
macro_rules! Depcrate_mut_mutimpl_7556 {
() => {
// Module: crate::mut_mut
// Provides: {"impl_7556"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MutMut { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx hir :: Block < '_ >) { intravisit :: walk_block (& mut MutVisitor { cx } , block) ; } fn check_ty (& mut self , cx : & LateContext < 'tcx > , ty : & 'tcx hir :: Ty < '_ , AmbigArg >) { if let hir :: TyKind :: Ref (_ , mty) = ty . kind && mty . mutbl == hir :: Mutability :: Mut && let hir :: TyKind :: Ref (_ , mty) = mty . ty . kind && mty . mutbl == hir :: Mutability :: Mut && ! ty . span . in_external_macro (cx . sess () . source_map ()) { span_lint (cx , MUT_MUT , ty . span , "generally you want to avoid `&mut &mut _` if possible" ,) ; } } }
};
}
