macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > Eq for StringInner < LenT , S > { }
    };
}

impl_254!()