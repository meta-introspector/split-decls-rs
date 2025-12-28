macro_rules! BreakableCtxt {
    () => {
        pub struct BreakableCtxt < 'tcx > { may_break : bool , coerce : Option < DynamicCoerceMany < 'tcx > > , }
    };
}

BreakableCtxt!()