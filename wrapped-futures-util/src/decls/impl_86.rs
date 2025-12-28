macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < Fut : Future > fmt :: Debug for Shared < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Shared") . field ("inner" , & self . inner) . field ("waker_key" , & self . waker_key) . finish () } }
    };
}

impl_86!()