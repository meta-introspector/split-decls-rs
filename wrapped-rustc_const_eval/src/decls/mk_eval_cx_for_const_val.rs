macro_rules! deps {
    () => {
        CompileTimeInterpCx!();
        CanAccessMutGlobal!();
        OpTy!();
    };
}

macro_rules! mk_eval_cx_for_const_val {
    () => {
        deps!();
        # [doc = " Create an interpreter context to inspect the given `ConstValue`."] # [doc = " Returns both the context and an `OpTy` that represents the constant."] pub fn mk_eval_cx_for_const_val < 'tcx > (tcx : TyCtxtAt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Option < (CompileTimeInterpCx < 'tcx > , OpTy < 'tcx >) > { let ecx = mk_eval_cx_to_read_const_val (tcx . tcx , tcx . span , typing_env , CanAccessMutGlobal :: No) ; let op = ecx . const_val_to_op (val , ty , None) . discard_err () ? ; Some ((ecx , op)) }
    };
}

mk_eval_cx_for_const_val!()