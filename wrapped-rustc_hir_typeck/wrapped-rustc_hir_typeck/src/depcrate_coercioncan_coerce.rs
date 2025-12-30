// Generated macro for can_coerce (function)
macro_rules! Depcrate_coercioncan_coerce {
() => {
// Module: crate::coercion
// Provides: {"can_coerce"}
// Dependencies: {}
# [doc = " Check whether `ty` can be coerced to `output_ty`."] # [doc = " Used from clippy."] pub fn can_coerce < 'tcx > (tcx : TyCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , body_id : LocalDefId , ty : Ty < 'tcx > , output_ty : Ty < 'tcx > ,) -> bool { let root_ctxt = crate :: typeck_root_ctxt :: TypeckRootCtxt :: new (tcx , body_id) ; let fn_ctxt = FnCtxt :: new (& root_ctxt , param_env , body_id) ; fn_ctxt . may_coerce (ty , output_ty) }
};
}
