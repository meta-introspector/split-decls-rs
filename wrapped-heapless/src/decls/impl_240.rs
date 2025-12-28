macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > fmt :: Display for StringInner < LenT , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < str as fmt :: Display > :: fmt (self , f) } }
    };
}

impl_240!();