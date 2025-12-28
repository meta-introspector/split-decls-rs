macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T > fmt :: Debug for CapacityError < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}: {}" , "CapacityError" , CAPERROR) } }
    };
}

impl_110!()