macro_rules! deps {
    () => {
        CanAccessMutGlobal!();
        CompileTimeMachine!();
        CompileTimeInterpCx!();
        CheckAlignment!();
        InterpCx!();
    };
}

macro_rules! mk_eval_cx_to_read_const_val {
    () => {
        deps!();
        # [doc = " The `InterpCx` is only meant to be used to do field and index projections into constants for"] # [doc = " `simd_shuffle` and const patterns in match arms."] # [doc = ""] # [doc = " This should *not* be used to do any actual interpretation. In particular, alignment checks are"] # [doc = " turned off!"] # [doc = ""] # [doc = " The function containing the `match` that is currently being analyzed may have generic bounds"] # [doc = " that inform us about the generic bounds of the constant. E.g., using an associated constant"] # [doc = " of a function's generic parameter will require knowledge about the bounds on the generic"] # [doc = " parameter. These bounds are passed to `mk_eval_cx` via the `ParamEnv` argument."] pub (crate) fn mk_eval_cx_to_read_const_val < 'tcx > (tcx : TyCtxt < 'tcx > , root_span : Span , typing_env : ty :: TypingEnv < 'tcx > , can_access_mut_global : CanAccessMutGlobal ,) -> CompileTimeInterpCx < 'tcx > { debug ! ("mk_eval_cx: {:?}" , typing_env) ; InterpCx :: new (tcx , root_span , typing_env , CompileTimeMachine :: new (can_access_mut_global , CheckAlignment :: No) ,) }
    };
}

mk_eval_cx_to_read_const_val!()