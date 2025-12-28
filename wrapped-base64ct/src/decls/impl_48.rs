macro_rules! deps {
    () => {
        InvalidEncodingError!();
        Error!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl fmt :: Display for InvalidEncodingError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (INVALID_ENCODING_MSG) } }
    };
}

impl_48!();