macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < T , LenT : LenType > FusedIterator for Drain < '_ , T , LenT > { }
    };
}

impl_280!()