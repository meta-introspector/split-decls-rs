// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl Return { fn return_single (& mut self , resolve : & Resolve , ty : & Type , orig_ty : & Type , sig_flattening : bool ,) { let id = match ty { Type :: Id (id) => * id , Type :: String => { self . retptrs . push (* orig_ty) ; return ; } Type :: ErrorContext => todo ! ("return_single for error-context") , _ => { self . scalar = Some (Scalar :: Type (* orig_ty)) ; return ; } } ; match & resolve . types [id] . kind { TypeDefKind :: Type (t) => return self . return_single (resolve , t , orig_ty , sig_flattening) , TypeDefKind :: Flags (_) | TypeDefKind :: Enum (_) | TypeDefKind :: Handle (_) | TypeDefKind :: Future (_) | TypeDefKind :: Stream (_) => { self . scalar = Some (Scalar :: Type (* orig_ty)) ; return ; } TypeDefKind :: Option (ty) => { if sig_flattening { self . scalar = Some (Scalar :: OptionBool (* ty)) ; self . retptrs . push (* ty) ; return ; } } TypeDefKind :: Result (r) => { if sig_flattening { if let Some (ok) = r . ok { self . retptrs . push (ok) ; } if let Some (err) = r . err { self . retptrs . push (err) ; } self . scalar = Some (Scalar :: ResultBool (r . ok , r . err)) ; return ; } } TypeDefKind :: Tuple (_) | TypeDefKind :: Record (_) | TypeDefKind :: List (_) | TypeDefKind :: Variant (_) => { } TypeDefKind :: Resource => todo ! ("return_single for resource") , TypeDefKind :: Unknown => unreachable ! () , TypeDefKind :: FixedSizeList (..) => todo ! () , } self . retptrs . push (* orig_ty) ; } }
};
}
