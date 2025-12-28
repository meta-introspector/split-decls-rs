macro_rules! deps {
    () => {
        SplitMut!();
    };
}

macro_rules! impl_1272 {
    () => {
        deps!();
        impl < T : Debug , P > Debug for SplitMut < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SplitMut") . field ("slice" , & self . slice) . finish () } }
    };
}

impl_1272!();