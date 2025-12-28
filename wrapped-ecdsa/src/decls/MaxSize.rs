macro_rules! deps {
    () => {
        MaxOverhead!();
    };
}

macro_rules! MaxSize {
    () => {
        deps!();
        # [doc = " Maximum size of an ASN.1 DER encoded signature for the given elliptic curve."] pub type MaxSize < C > = < < FieldBytesSize < C > as Add > :: Output as Add < MaxOverhead > > :: Output ;
    };
}

MaxSize!();