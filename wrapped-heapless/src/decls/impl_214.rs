macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < LenT : LenType > AsRef < [u8] > for Drain < '_ , LenT > { fn as_ref (& self) -> & [u8] { self . as_str () . as_bytes () } }
    };
}

impl_214!()