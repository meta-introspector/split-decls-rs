macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , S , A : Allocator > Debug for OccupiedEntry < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
    };
}

impl_284!();