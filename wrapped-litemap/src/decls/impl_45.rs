macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < K , V , S > Debug for Entry < '_ , K , V , S > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Occupied (arg0) => f . debug_tuple ("Occupied") . field (arg0) . finish () , Self :: Vacant (arg0) => f . debug_tuple ("Vacant") . field (arg0) . finish () , } } }
    };
}

impl_45!();