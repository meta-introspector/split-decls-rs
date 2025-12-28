macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < T : fmt :: Debug , S , A : Allocator > fmt :: Debug for OccupiedEntry < '_ , T , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("value" , self . get ()) . finish () } }
    };
}

impl_459!()