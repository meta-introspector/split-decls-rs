macro_rules! deps {
    () => {
        AbsentEntry!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for AbsentEntry < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("AbsentEntry") } }
    };
}

impl_489!()