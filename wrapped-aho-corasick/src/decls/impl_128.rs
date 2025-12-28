macro_rules! deps {
    () => {
        Pattern!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Pattern < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pattern") . field ("lit" , & String :: from_utf8_lossy (self . 0)) . finish () } }
    };
}

impl_128!()