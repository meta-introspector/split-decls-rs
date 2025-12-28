macro_rules! deps {
    () => {
        InferCtxt!();
        OpportunisticVarResolver!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'a , 'tcx > OpportunisticVarResolver < 'a , 'tcx > { # [inline] pub fn new (infcx : & 'a InferCtxt < 'tcx >) -> Self { OpportunisticVarResolver { infcx , cache : Default :: default () } } }
    };
}

impl_178!()