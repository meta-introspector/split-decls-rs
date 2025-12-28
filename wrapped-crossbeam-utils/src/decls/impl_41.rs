macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T : Copy + fmt :: Debug > fmt :: Debug for AtomicCell < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AtomicCell") . field ("value" , & self . load ()) . finish () } }
    };
}

impl_41!();