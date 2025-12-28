macro_rules! deps {
    () => {
        InferCtxt!();
        OpportunisticRegionResolver!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < 'a , 'tcx > OpportunisticRegionResolver < 'a , 'tcx > { pub fn new (infcx : & 'a InferCtxt < 'tcx >) -> Self { OpportunisticRegionResolver { infcx } } }
    };
}

impl_181!();