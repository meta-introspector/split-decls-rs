macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        unsafe impl < LenT : LenType > Sync for Drain < '_ , LenT > { }
    };
}

impl_209!()