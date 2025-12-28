macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < T , LenT : LenType > AsRef < [T] > for Drain < '_ , T , LenT > { fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_273!();