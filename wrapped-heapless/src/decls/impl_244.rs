macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > ops :: DerefMut for StringInner < LenT , S > { fn deref_mut (& mut self) -> & mut str { self . as_mut_str () } }
    };
}

impl_244!();