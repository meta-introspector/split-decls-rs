macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > fmt :: Debug for StringInner < LenT , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < str as fmt :: Debug > :: fmt (self , f) } }
    };
}

impl_239!();