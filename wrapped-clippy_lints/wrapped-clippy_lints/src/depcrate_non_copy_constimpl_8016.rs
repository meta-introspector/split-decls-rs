// Generated macro for impl_8016 (impl)
macro_rules! Depcrate_non_copy_constimpl_8016 {
() => {
// Module: crate::non_copy_const
// Provides: {"impl_8016"}
// Dependencies: {}
impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for ReplaceAssocFolder < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if let ty :: Alias (AliasTyKind :: Projection , ty) = ty . kind () && ty . trait_def_id (self . tcx) == self . trait_id && ty . self_ty () == self . self_ty { self . tcx . types . unit } else { ty . super_fold_with (self) } } }
};
}
