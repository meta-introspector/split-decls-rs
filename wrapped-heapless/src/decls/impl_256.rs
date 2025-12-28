macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > Ord for StringInner < LenT , S > { # [inline] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
    };
}

impl_256!()