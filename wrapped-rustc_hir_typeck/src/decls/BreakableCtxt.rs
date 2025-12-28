macro_rules! deps {
    () => {
        DynamicCoerceMany!();
    };
}

macro_rules! BreakableCtxt {
    () => {
        deps!();
        pub struct BreakableCtxt < 'tcx > { may_break : bool , coerce : Option < DynamicCoerceMany < 'tcx > > , }
    };
}

BreakableCtxt!();