// Generated macro for impl_3353 (impl)
macro_rules! Depcrate_len_zeroimpl_3353 {
() => {
// Module: crate::len_zero
// Provides: {"impl_3353"}
// Dependencies: {}
impl LenOutput { fn matches_is_empty_output < 'tcx > (self , cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { if let Some (segment) = extract_future_output (cx , ty) { return match (self , segment . res) { (_ , Res :: PrimTy (PrimTy :: Bool)) => true , (Self :: Option (_) , Res :: Def (_ , def_id)) if cx . tcx . is_diagnostic_item (sym :: Option , def_id) => true , (Self :: Result (_) , Res :: Def (_ , def_id)) if cx . tcx . is_diagnostic_item (sym :: Result , def_id) => true , _ => false , } ; } match (self , ty . kind ()) { (_ , & ty :: Bool) => true , (Self :: Option (id) , & ty :: Adt (adt , subs)) if id == adt . did () => subs . type_at (0) . is_bool () , (Self :: Result (id) , & ty :: Adt (adt , subs)) if id == adt . did () => subs . type_at (0) . is_bool () , _ => false , } } fn expected_sig (self , self_kind : ImplicitSelfKind) -> String { let self_ref = match self_kind { ImplicitSelfKind :: RefImm => "&" , ImplicitSelfKind :: RefMut => "&mut " , _ => "" , } ; match self { Self :: Integral => format ! ("expected signature: `({self_ref}self) -> bool`") , Self :: Option (_) => { format ! ("expected signature: `({self_ref}self) -> bool` or `({self_ref}self) -> Option<bool>") } , Self :: Result (..) => { format ! ("expected signature: `({self_ref}self) -> bool` or `({self_ref}self) -> Result<bool>") } , } } }
};
}
