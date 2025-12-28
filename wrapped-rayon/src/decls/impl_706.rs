macro_rules! deps {
    () => {
        MapInit!();
    };
}

macro_rules! impl_706 {
    () => {
        deps!();
        impl < I : Debug , INIT , F > Debug for MapInit < I , INIT , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapInit") . field ("base" , & self . base) . finish () } }
    };
}

impl_706!()