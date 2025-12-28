macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for OccupiedEntry < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("value" , self . get ()) . finish () } }
    };
}

impl_483!();