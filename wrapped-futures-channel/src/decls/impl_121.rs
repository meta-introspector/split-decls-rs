macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Receiver < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Receiver") . field ("complete" , & self . inner . complete) . finish () } }
    };
}

impl_121!()