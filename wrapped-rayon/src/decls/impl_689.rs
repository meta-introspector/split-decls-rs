macro_rules! deps {
    () => {
        MapWith!();
    };
}

macro_rules! impl_689 {
    () => {
        deps!();
        impl < I : Debug , T : Debug , F > Debug for MapWith < I , T , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapWith") . field ("base" , & self . base) . field ("item" , & self . item) . finish () } }
    };
}

impl_689!();