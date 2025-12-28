macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for VacantEntry < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("VacantEntry") } }
    };
}

impl_486!()