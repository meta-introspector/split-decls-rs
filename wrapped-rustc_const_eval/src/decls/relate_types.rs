macro_rules! relate_types {
    () => {
        # [doc = " Returns whether `src` is a subtype of `dest`, i.e. `src <: dest`."] # [doc = ""] # [doc = " When validating assignments, the variance should be `Covariant`. When checking"] # [doc = " during `MirPhase` >= `MirPhase::Runtime(RuntimePhase::Initial)` variance should be `Invariant`"] # [doc = " because we want to check for type equality."] pub fn relate_types < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , variance : Variance , src : Ty < 'tcx > , dest : Ty < 'tcx > ,) -> bool { if src == dest { return true ; } let (infcx , param_env) = tcx . infer_ctxt () . ignoring_regions () . build_with_typing_env (typing_env) ; let ocx = ObligationCtxt :: new (& infcx) ; let cause = ObligationCause :: dummy () ; let src = ocx . normalize (& cause , param_env , src) ; let dest = ocx . normalize (& cause , param_env , dest) ; match ocx . relate (& cause , param_env , variance , src , dest) { Ok (()) => { } Err (_) => return false , } ; ocx . select_all_or_error () . is_empty () }
    };
}

relate_types!()