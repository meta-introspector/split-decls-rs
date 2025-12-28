macro_rules! deps {
    () => {
        DivergingBlockBehavior!();
        DivergingFallbackBehavior!();
    };
}

macro_rules! never_type_behavior {
    () => {
        deps!();
        fn never_type_behavior (tcx : TyCtxt < '_ >) -> (DivergingFallbackBehavior , DivergingBlockBehavior) { let (fallback , block) = parse_never_type_options_attr (tcx) ; let fallback = fallback . unwrap_or_else (| | default_fallback (tcx)) ; let block = block . unwrap_or_default () ; (fallback , block) }
    };
}

never_type_behavior!()