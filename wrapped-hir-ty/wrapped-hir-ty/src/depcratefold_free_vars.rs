// Generated macro for fold_free_vars (function)
macro_rules! Depcratefold_free_vars {
() => {
// Module: crate
// Provides: {"fold_free_vars"}
// Dependencies: {}
pub (crate) fn fold_free_vars < T : HasInterner < Interner = Interner > + TypeFoldable < Interner > > (t : T , for_ty : impl FnMut (BoundVar , DebruijnIndex) -> Ty , for_const : impl FnMut (Ty , BoundVar , DebruijnIndex) -> Const ,) -> T { use chalk_ir :: fold :: TypeFolder ; # [derive (chalk_derive :: FallibleTypeFolder)] # [has_interner (Interner)] struct FreeVarFolder < F1 : FnMut (BoundVar , DebruijnIndex) -> Ty , F2 : FnMut (Ty , BoundVar , DebruijnIndex) -> Const , > (F1 , F2) ; impl < F1 : FnMut (BoundVar , DebruijnIndex) -> Ty , F2 : FnMut (Ty , BoundVar , DebruijnIndex) -> Const > TypeFolder < Interner > for FreeVarFolder < F1 , F2 > { fn as_dyn (& mut self) -> & mut dyn TypeFolder < Interner > { self } fn interner (& self) -> Interner { Interner } fn fold_free_var_ty (& mut self , bound_var : BoundVar , outer_binder : DebruijnIndex) -> Ty { self . 0 (bound_var , outer_binder) } fn fold_free_var_const (& mut self , ty : Ty , bound_var : BoundVar , outer_binder : DebruijnIndex ,) -> Const { self . 1 (ty , bound_var , outer_binder) } } t . fold_with (& mut FreeVarFolder (for_ty , for_const) , DebruijnIndex :: INNERMOST) }
};
}
