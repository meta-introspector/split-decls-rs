macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , S , A : Allocator > Debug for Entry < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
    };
}

impl_280!()