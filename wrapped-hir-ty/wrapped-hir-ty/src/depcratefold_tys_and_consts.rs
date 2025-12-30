// Generated macro for fold_tys_and_consts (function)
macro_rules! Depcratefold_tys_and_consts {
() => {
// Module: crate
// Provides: {"fold_tys_and_consts"}
// Dependencies: {}
pub (crate) fn fold_tys_and_consts < T : HasInterner < Interner = Interner > + TypeFoldable < Interner > > (t : T , f : impl FnMut (Either < Ty , Const > , DebruijnIndex) -> Either < Ty , Const > , binders : DebruijnIndex ,) -> T { use chalk_ir :: fold :: { TypeFolder , TypeSuperFoldable } ; # [derive (chalk_derive :: FallibleTypeFolder)] # [has_interner (Interner)] struct TyFolder < F : FnMut (Either < Ty , Const > , DebruijnIndex) -> Either < Ty , Const > > (F) ; impl < F : FnMut (Either < Ty , Const > , DebruijnIndex) -> Either < Ty , Const > > TypeFolder < Interner > for TyFolder < F > { fn as_dyn (& mut self) -> & mut dyn TypeFolder < Interner > { self } fn interner (& self) -> Interner { Interner } fn fold_ty (& mut self , ty : Ty , outer_binder : DebruijnIndex) -> Ty { let ty = ty . super_fold_with (self . as_dyn () , outer_binder) ; self . 0 (Either :: Left (ty) , outer_binder) . left () . unwrap () } fn fold_const (& mut self , c : Const , outer_binder : DebruijnIndex) -> Const { self . 0 (Either :: Right (c) , outer_binder) . right () . unwrap () } } t . fold_with (& mut TyFolder (f) , binders) }
};
}
