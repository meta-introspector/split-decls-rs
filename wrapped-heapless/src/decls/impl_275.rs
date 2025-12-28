macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        unsafe impl < T : Send , LenT : LenType > Send for Drain < '_ , T , LenT > { }
    };
}

impl_275!();