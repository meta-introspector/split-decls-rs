macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        unsafe impl < LenT : LenType > Send for Drain < '_ , LenT > { }
    };
}

impl_210!()