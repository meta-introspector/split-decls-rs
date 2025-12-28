macro_rules! deps {
    () => {
        Filter!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < I : Debug , P > Debug for Filter < I , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Filter") . field ("base" , & self . base) . finish () } }
    };
}

impl_459!()