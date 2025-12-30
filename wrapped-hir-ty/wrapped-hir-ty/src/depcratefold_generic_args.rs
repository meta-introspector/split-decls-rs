// Generated macro for fold_generic_args (function)
macro_rules! Depcratefold_generic_args {
() => {
// Module: crate
// Provides: {"fold_generic_args"}
// Dependencies: {}
pub (crate) fn fold_generic_args < T : HasInterner < Interner = Interner > + TypeFoldable < Interner > > (t : T , f : impl FnMut (GenericArgData , DebruijnIndex) -> GenericArgData , binders : DebruijnIndex ,) -> T { use chalk_ir :: fold :: { TypeFolder , TypeSuperFoldable } ; # [derive (chalk_derive :: FallibleTypeFolder)] # [has_interner (Interner)] struct TyFolder < F : FnMut (GenericArgData , DebruijnIndex) -> GenericArgData > (F) ; impl < F : FnMut (GenericArgData , DebruijnIndex) -> GenericArgData > TypeFolder < Interner > for TyFolder < F > { fn as_dyn (& mut self) -> & mut dyn TypeFolder < Interner > { self } fn interner (& self) -> Interner { Interner } fn fold_ty (& mut self , ty : Ty , outer_binder : DebruijnIndex) -> Ty { let ty = ty . super_fold_with (self . as_dyn () , outer_binder) ; self . 0 (GenericArgData :: Ty (ty) , outer_binder) . intern (Interner) . ty (Interner) . unwrap () . clone () } fn fold_const (& mut self , c : Const , outer_binder : DebruijnIndex) -> Const { self . 0 (GenericArgData :: Const (c) , outer_binder) . intern (Interner) . constant (Interner) . unwrap () . clone () } fn fold_lifetime (& mut self , lt : Lifetime , outer_binder : DebruijnIndex) -> Lifetime { let lt = lt . super_fold_with (self . as_dyn () , outer_binder) ; self . 0 (GenericArgData :: Lifetime (lt) , outer_binder) . intern (Interner) . lifetime (Interner) . unwrap () . clone () } } t . fold_with (& mut TyFolder (f) , binders) }
};
}
