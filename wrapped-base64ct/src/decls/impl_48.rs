macro_rules! deps {
    () => {
        Error!();
        InvalidEncodingError!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl fmt :: Display for InvalidEncodingError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (INVALID_ENCODING_MSG) } }
    };
}

impl_48!()