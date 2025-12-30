// Generated macro for get_path_to_ty (function)
macro_rules! Depcrateget_path_to_ty {
() => {
// Module: crate
// Provides: {"get_path_to_ty"}
// Dependencies: {}
fn get_path_to_ty < 'tcx > (tcx : TyCtxt < 'tcx > , from : LocalDefId , ty : Ty < 'tcx > , args : GenericArgsRef < 'tcx >) -> String { match ty . kind () { rustc_ty :: Adt (adt , _) => get_path_to_callee (tcx , from , adt . did ()) , rustc_ty :: Array (..) | rustc_ty :: Dynamic (..) | rustc_ty :: Never | rustc_ty :: RawPtr (_ , _) | rustc_ty :: Ref (..) | rustc_ty :: Slice (_) | rustc_ty :: Tuple (_) => format ! ("<{}>" , EarlyBinder :: bind (ty) . instantiate (tcx , args)) , _ => ty . to_string () , } }
};
}
