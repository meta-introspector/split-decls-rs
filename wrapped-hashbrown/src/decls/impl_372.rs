macro_rules! deps {
    () => {
        RustcVacantEntry!();
        VacantEntry!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl < K : Debug , V , A : Allocator > Debug for RustcVacantEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
    };
}

impl_372!();