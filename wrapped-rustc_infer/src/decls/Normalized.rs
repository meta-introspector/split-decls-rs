macro_rules! deps {
    () => {
        PredicateObligations!();
    };
}

macro_rules! Normalized {
    () => {
        deps!();
        # [derive (Clone)] pub struct Normalized < 'tcx , T > { pub value : T , pub obligations : PredicateObligations < 'tcx > , }
    };
}

Normalized!()