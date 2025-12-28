macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        unsafe impl < T : Sync , LenT : LenType > Sync for Drain < '_ , T , LenT > { }
    };
}

impl_274!()