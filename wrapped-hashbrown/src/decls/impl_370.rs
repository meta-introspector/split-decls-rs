macro_rules! deps {
    () => {
        RustcOccupiedEntry!();
        OccupiedEntry!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , A : Allocator > Debug for RustcOccupiedEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
    };
}

impl_370!()