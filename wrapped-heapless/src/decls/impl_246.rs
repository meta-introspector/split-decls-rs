macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > borrow :: BorrowMut < str > for StringInner < LenT , S > { fn borrow_mut (& mut self) -> & mut str { self . as_mut_str () } }
    };
}

impl_246!();