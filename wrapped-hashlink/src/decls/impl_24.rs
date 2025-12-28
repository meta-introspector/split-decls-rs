macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for Entry < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
    };
}

impl_24!()