macro_rules! deps {
    () => {
        LinearMapInner!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < K , V , S : LinearMapStorage < K , V > + ? Sized > fmt :: Debug for LinearMapInner < K , V , S > where K : Eq + fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
    };
}

impl_175!();