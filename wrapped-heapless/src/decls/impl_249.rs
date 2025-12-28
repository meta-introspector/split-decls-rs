macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < LenT1 : LenType , LenT2 : LenType , S1 : StringStorage + ? Sized , S2 : StringStorage + ? Sized > PartialEq < StringInner < LenT1 , S1 > > for StringInner < LenT2 , S2 > { fn eq (& self , rhs : & StringInner < LenT1 , S1 >) -> bool { str :: eq (& * * self , & * * rhs) } }
    };
}

impl_249!();