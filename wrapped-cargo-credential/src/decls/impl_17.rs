macro_rules! deps {
    () => {
        Secret!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Secret < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Secret") . field ("inner" , & "REDACTED") . finish () } }
    };
}

impl_17!()