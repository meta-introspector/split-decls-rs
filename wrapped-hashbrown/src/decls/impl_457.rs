macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        impl < T : fmt :: Debug , S , A : Allocator > fmt :: Debug for Entry < '_ , T , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
    };
}

impl_457!()