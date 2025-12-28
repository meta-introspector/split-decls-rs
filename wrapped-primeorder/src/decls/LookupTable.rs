macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! LookupTable {
    () => {
        deps!();
        struct LookupTable < C : PrimeCurveParams > ([ProjectivePoint < C > ; 16]) ;
    };
}

LookupTable!();