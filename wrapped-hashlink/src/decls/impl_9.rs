macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < K , V , S > fmt :: Debug for LinkedHashMap < K , V , S > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entries (self) . finish () } }
    };
}

impl_9!();