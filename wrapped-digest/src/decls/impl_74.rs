macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T : ExtendableOutput + fmt :: Debug , S : ArraySize > fmt :: Debug for XofFixedWrapper < T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("XofFixedWrapper") . field ("hash" , & self . hash) . field ("_size" , & self . size) . finish () } }
    };
}

impl_74!()