macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < K , V , A > fmt :: Debug for Drain < '_ , K , V , A > where K : fmt :: Debug , V : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_327!()