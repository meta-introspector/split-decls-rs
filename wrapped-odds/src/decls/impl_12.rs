macro_rules! deps {
    () => {
        EncodeUtf8Error!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Display for EncodeUtf8Error { # [inline] fn fmt (& self , fmtr : & mut Formatter) -> fmt :: Result { fmtr . pad (self . description ()) } }
    };
}

impl_12!();