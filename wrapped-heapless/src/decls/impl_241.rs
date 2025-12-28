macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > hash :: Hash for StringInner < LenT , S > { # [inline] fn hash < H : hash :: Hasher > (& self , hasher : & mut H) { < str as hash :: Hash > :: hash (self , hasher) ; } }
    };
}

impl_241!()