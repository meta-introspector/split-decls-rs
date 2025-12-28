macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , A : Allocator > fmt :: Debug for IntoIter < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_310!();