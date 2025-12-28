macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > ops :: Deref for StringInner < LenT , S > { type Target = str ; fn deref (& self) -> & str { self . as_str () } }
    };
}

impl_243!();