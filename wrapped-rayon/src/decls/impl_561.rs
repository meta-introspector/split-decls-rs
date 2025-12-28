macro_rules! deps {
    () => {
        FoldWith!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl < I : Debug , U : Debug , F > Debug for FoldWith < I , U , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FoldWith") . field ("base" , & self . base) . field ("item" , & self . item) . finish () } }
    };
}

impl_561!();