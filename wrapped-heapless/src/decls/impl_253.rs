macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < StringInner < LenT , S > > for & str { # [inline] fn eq (& self , other : & StringInner < LenT , S >) -> bool { str :: eq (self , & other [..]) } }
    };
}

impl_253!();