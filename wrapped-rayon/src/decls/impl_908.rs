macro_rules! deps {
    () => {
        TryFoldWith!();
    };
}

macro_rules! impl_908 {
    () => {
        deps!();
        impl < I , U , F > Debug for TryFoldWith < I , U , F > where I : Debug , U : Try < Output : Debug > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFoldWith") . field ("base" , & self . base) . field ("item" , & self . item) . finish () } }
    };
}

impl_908!();