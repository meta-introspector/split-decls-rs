macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < T , LenT : LenType > ExactSizeIterator for Drain < '_ , T , LenT > { }
    };
}

impl_279!()