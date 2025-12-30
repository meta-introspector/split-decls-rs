// Generated macro for impl_7701 (impl)
macro_rules! Depcrate_mut_mutimpl_7701 {
() => {
// Module: crate::mut_mut
// Provides: {"impl_7701"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MutMut { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx hir :: Block < '_ >) { intravisit :: walk_block (& mut MutVisitor { cx } , block) ; } fn check_ty (& mut self , cx : & LateContext < 'tcx > , ty : & 'tcx hir :: Ty < '_ , AmbigArg >) { if let TyKind :: Ref (_ , mty) = ty . kind && mty . mutbl == Mutability :: Mut && let TyKind :: Ref (_ , mty2) = mty . ty . kind && mty2 . mutbl == Mutability :: Mut && ! ty . span . in_external_macro (cx . sess () . source_map ()) { if self . seen_tys . contains (& ty . hir_id) { return ; } let (mut t , mut t2) = (mty . ty , mty2 . ty) ; let mut many_muts = false ; loop { self . seen_tys . insert (t . hir_id) ; if let TyKind :: Ref (_ , next) = t2 . kind && next . mutbl == Mutability :: Mut { (t , t2) = (t2 , next . ty) ; many_muts = true ; } else { break ; } } let mut applicability = Applicability :: MaybeIncorrect ; let sugg = snippet_with_applicability (cx . sess () , t . span , ".." , & mut applicability) ; let suffix = if many_muts { "s" } else { "" } ; span_lint_and_sugg (cx , MUT_MUT , ty . span , "a type of form `&mut &mut _`" , format ! ("remove the extra `&mut`{suffix}") , sugg . to_string () , applicability ,) ; } } }
};
}
