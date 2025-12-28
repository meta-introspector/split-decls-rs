macro_rules! deps {
    () => {
        LruCache!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < K , V , S > fmt :: Debug for LruCache < K , V , S > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entries (self . iter () . rev ()) . finish () } }
    };
}

impl_191!();