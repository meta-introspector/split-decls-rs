// Generated macro for is_ref_dyn_any (function)
macro_rules! Depcrate_coerce_container_to_anyis_ref_dyn_any {
() => {
// Module: crate::coerce_container_to_any
// Provides: {"is_ref_dyn_any"}
// Dependencies: {}
fn is_ref_dyn_any (tcx : TyCtxt < '_ > , ty : Ty < '_ >) -> bool { let ty :: Ref (_ , ref_ty , _) = * ty . kind () else { return false ; } ; is_dyn_any (tcx , ref_ty) }
};
}
