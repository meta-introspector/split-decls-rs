macro_rules! deps {
    () => {
        TypeckRootCtxt!();
        FnCtxt!();
    };
}

macro_rules! can_coerce {
    () => {
        deps!();
        # [doc = " Check whether `ty` can be coerced to `output_ty`."] # [doc = " Used from clippy."] pub fn can_coerce < 'tcx > (tcx : TyCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , body_id : LocalDefId , ty : Ty < 'tcx > , output_ty : Ty < 'tcx > ,) -> bool { let root_ctxt = crate :: typeck_root_ctxt :: TypeckRootCtxt :: new (tcx , body_id) ; let fn_ctxt = FnCtxt :: new (& root_ctxt , param_env , body_id) ; fn_ctxt . may_coerce (ty , output_ty) }
    };
}

can_coerce!();