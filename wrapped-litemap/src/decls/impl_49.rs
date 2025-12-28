macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < K , V , S > Debug for VacantEntry < '_ , K , V , S > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("VacantEntry") . field ("index" , & self . index) . finish () } }
    };
}

impl_49!();