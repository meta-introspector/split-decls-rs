macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { let s = match self { Self :: InvalidEncoding => INVALID_ENCODING_MSG , Self :: InvalidLength => INVALID_LENGTH_MSG , } ; f . write_str (s) } }
    };
}

impl_51!();