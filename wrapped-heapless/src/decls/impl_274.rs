macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        unsafe impl < T : Sync , LenT : LenType > Sync for Drain < '_ , T , LenT > { }
    };
}

impl_274!();