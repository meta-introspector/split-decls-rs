macro_rules! deps {
    () => {
        RustcEntry!();
        Entry!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , A : Allocator > Debug for RustcEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
    };
}

impl_366!()