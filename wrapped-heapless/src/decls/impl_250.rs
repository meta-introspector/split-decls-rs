macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < str > for StringInner < LenT , S > { # [inline] fn eq (& self , other : & str) -> bool { str :: eq (self , other) } }
    };
}

impl_250!()