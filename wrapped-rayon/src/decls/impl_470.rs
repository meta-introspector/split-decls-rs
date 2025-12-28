macro_rules! deps {
    () => {
        FilterMap!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < I : Debug , P > Debug for FilterMap < I , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FilterMap") . field ("base" , & self . base) . finish () } }
    };
}

impl_470!()