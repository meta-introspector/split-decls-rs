macro_rules! deps {
    () => {
        FutureGroup!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < T : Debug > Debug for FutureGroup < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FutureGroup") . field ("slab" , & "[..]") . field ("len" , & self . len ()) . field ("capacity" , & self . capacity) . finish () } }
    };
}

impl_199!();