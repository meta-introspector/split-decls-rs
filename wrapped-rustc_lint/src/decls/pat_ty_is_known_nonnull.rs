macro_rules! deps {
    () => {
        PatternKind!();
    };
}

macro_rules! pat_ty_is_known_nonnull {
    () => {
        deps!();
        fn pat_ty_is_known_nonnull < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , pat : ty :: Pattern < 'tcx > ,) -> bool { Option :: unwrap_or_default (try { match * pat { ty :: PatternKind :: Range { start , end } => { let start = start . try_to_value () ? . try_to_bits (tcx , typing_env) ? ; let end = end . try_to_value () ? . try_to_bits (tcx , typing_env) ? ; start > 0 && end >= start } ty :: PatternKind :: Or (patterns) => { patterns . iter () . all (| pat | pat_ty_is_known_nonnull (tcx , typing_env , pat)) } } } ,) }
    };
}

pat_ty_is_known_nonnull!();