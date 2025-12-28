macro_rules! deps {
    () => {
        CastCheck!();
        FnCtxt!();
        TypeckRootCtxt!();
    };
}

macro_rules! check_cast {
    () => {
        deps!();
        # [doc = " If a cast from `from_ty` to `to_ty` is valid, returns a `Some` containing the kind"] # [doc = " of the cast."] # [doc = ""] # [doc = " This is a helper used from clippy."] pub fn check_cast < 'tcx > (tcx : TyCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , e : & 'tcx hir :: Expr < 'tcx > , from_ty : Ty < 'tcx > , to_ty : Ty < 'tcx > ,) -> Option < CastKind > { let hir_id = e . hir_id ; let local_def_id = hir_id . owner . def_id ; let root_ctxt = crate :: TypeckRootCtxt :: new (tcx , local_def_id) ; let fn_ctxt = FnCtxt :: new (& root_ctxt , param_env , local_def_id) ; if let Ok (check) = CastCheck :: new (& fn_ctxt , e , from_ty , to_ty , DUMMY_SP , DUMMY_SP ,) { check . do_check (& fn_ctxt) . ok () } else { None } }
    };
}

check_cast!()