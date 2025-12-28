macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < K , V , S > Debug for OccupiedEntry < '_ , K , V , S > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("index" , & self . index) . finish () } }
    };
}

impl_47!();