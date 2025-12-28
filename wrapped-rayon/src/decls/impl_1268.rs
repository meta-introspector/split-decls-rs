macro_rules! deps {
    () => {
        SplitInclusive!();
    };
}

macro_rules! impl_1268 {
    () => {
        deps!();
        impl < T : Debug , P > Debug for SplitInclusive < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SplitInclusive") . field ("slice" , & self . slice) . finish () } }
    };
}

impl_1268!();