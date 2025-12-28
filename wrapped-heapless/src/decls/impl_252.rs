macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < StringInner < LenT , S > > for str { # [inline] fn eq (& self , other : & StringInner < LenT , S >) -> bool { Self :: eq (self , & other [..]) } }
    };
}

impl_252!();