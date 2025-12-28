macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for OccupiedEntry < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
    };
}

impl_27!();