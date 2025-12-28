macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < K , V , S > fmt :: Debug for IndexMap < K , V , S > where K : fmt :: Debug , V : fmt :: Debug , { # [cfg (not (feature = "test_debug"))] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } # [cfg (feature = "test_debug")] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IndexMap") . field ("core" , & self . core) . finish () } }
    };
}

impl_50!()