macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < & str > for StringInner < LenT , S > { # [inline] fn eq (& self , other : & & str) -> bool { str :: eq (self , & other [..]) } }
    };
}

impl_251!();