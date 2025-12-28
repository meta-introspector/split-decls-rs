macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        unsafe impl < LenT : LenType > Send for Drain < '_ , LenT > { }
    };
}

impl_210!();