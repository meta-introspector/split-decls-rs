macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < LenT : LenType > AsRef < str > for Drain < '_ , LenT > { fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_213!();