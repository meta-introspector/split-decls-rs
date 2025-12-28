macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > AsRef < str > for StringInner < LenT , S > { # [inline] fn as_ref (& self) -> & str { self } }
    };
}

impl_247!()