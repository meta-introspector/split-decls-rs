macro_rules! deps {
    () => {
        OpportunisticVarResolver!();
        InferCtxt!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'a , 'tcx > OpportunisticVarResolver < 'a , 'tcx > { # [inline] pub fn new (infcx : & 'a InferCtxt < 'tcx >) -> Self { OpportunisticVarResolver { infcx , cache : Default :: default () } } }
    };
}

impl_178!();