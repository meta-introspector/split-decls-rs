macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < K : Debug , V , S , A : Allocator > Debug for VacantEntry < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
    };
}

impl_286!();