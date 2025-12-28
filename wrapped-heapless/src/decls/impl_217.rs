macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < LenT : LenType > FusedIterator for Drain < '_ , LenT > { }
    };
}

impl_217!();