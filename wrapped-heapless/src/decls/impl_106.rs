macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > fmt :: Debug for IndexMap < K , V , S , N > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
    };
}

impl_106!();