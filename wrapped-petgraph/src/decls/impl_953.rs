macro_rules! deps {
    () => {
        DebugMap!();
    };
}

macro_rules! impl_953 {
    () => {
        deps!();
        impl < F , I , K , V > fmt :: Debug for DebugMap < F > where F : Fn () -> I , I : IntoIterator < Item = (K , V) > , K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entries ((self . 0) ()) . finish () } }
    };
}

impl_953!()