macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_726 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for Signature < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} <{}>" , String :: from_utf8_lossy (self . name_bytes ()) , String :: from_utf8_lossy (self . email_bytes ())) } }
    };
}

impl_726!();