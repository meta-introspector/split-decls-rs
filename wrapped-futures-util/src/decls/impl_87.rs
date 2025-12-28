macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < Fut : Future > fmt :: Debug for Inner < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Inner") . finish () } }
    };
}

impl_87!()