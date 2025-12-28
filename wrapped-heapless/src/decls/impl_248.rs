macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > AsRef < [u8] > for StringInner < LenT , S > { # [inline] fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_248!();