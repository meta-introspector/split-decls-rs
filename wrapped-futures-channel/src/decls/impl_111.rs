macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Sender < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Sender") . field ("complete" , & self . inner . complete) . finish () } }
    };
}

impl_111!();