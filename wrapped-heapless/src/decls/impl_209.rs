macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        unsafe impl < LenT : LenType > Sync for Drain < '_ , LenT > { }
    };
}

impl_209!();