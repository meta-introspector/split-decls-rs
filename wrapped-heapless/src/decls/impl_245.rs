macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > borrow :: Borrow < str > for StringInner < LenT , S > { fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_245!()