// Generated macro for clif_pair_type_from_ty (function)
macro_rules! Depcrate_commonclif_pair_type_from_ty {
() => {
// Module: crate::common
// Provides: {"clif_pair_type_from_ty"}
// Dependencies: {}
fn clif_pair_type_from_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > ,) -> Option < (types :: Type , types :: Type) > { Some (match ty . kind () { ty :: Tuple (types) if types . len () == 2 => { (clif_type_from_ty (tcx , types [0]) ? , clif_type_from_ty (tcx , types [1]) ?) } ty :: RawPtr (pointee_ty , _) | ty :: Ref (_ , pointee_ty , _) => { if tcx . type_has_metadata (* pointee_ty , ty :: TypingEnv :: fully_monomorphized ()) { (pointer_ty (tcx) , pointer_ty (tcx)) } else { return None ; } } _ => return None , }) }
};
}
