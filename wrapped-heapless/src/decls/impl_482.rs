macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > uDisplay for StringInner < LenT , S > { # [inline] fn fmt < W > (& self , f : & mut ufmt :: Formatter < '_ , W >) -> Result < () , W :: Error > where W : uWrite + ? Sized , { f . write_str (self . as_str ()) } }
    };
}

impl_482!();