macro_rules! deps {
    () => {
        SplitInclusiveMut!();
    };
}

macro_rules! impl_1275 {
    () => {
        deps!();
        impl < T : Debug , P > Debug for SplitInclusiveMut < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SplitInclusiveMut") . field ("slice" , & self . slice) . finish () } }
    };
}

impl_1275!()