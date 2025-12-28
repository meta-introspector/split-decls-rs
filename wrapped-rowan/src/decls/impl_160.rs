macro_rules! deps {
    () => {
        ThinArc!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < H : Hash , T : Hash > Hash for ThinArc < H , T > { fn hash < HSR : Hasher > (& self , state : & mut HSR) { (* * self) . hash (state) } }
    };
}

impl_160!();