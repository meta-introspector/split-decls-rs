macro_rules! deps {
    () => {
        PatternKind!();
    };
}

macro_rules! get_nullable_type_from_pat {
    () => {
        deps!();
        fn get_nullable_type_from_pat < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , base : Ty < 'tcx > , pat : ty :: Pattern < 'tcx > ,) -> Option < Ty < 'tcx > > { match * pat { ty :: PatternKind :: Range { .. } => get_nullable_type (tcx , typing_env , base) , ty :: PatternKind :: Or (patterns) => { let first = get_nullable_type_from_pat (tcx , typing_env , base , patterns [0]) ? ; for & pat in & patterns [1 ..] { assert_eq ! (first , get_nullable_type_from_pat (tcx , typing_env , base , pat) ?) ; } Some (first) } } }
    };
}

get_nullable_type_from_pat!();