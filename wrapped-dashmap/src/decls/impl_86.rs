macro_rules! deps {
    () => {
        ReadOnlyView!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < K : Eq + Hash + fmt :: Debug , V : fmt :: Debug , S : BuildHasher + Clone > fmt :: Debug for ReadOnlyView < K , V , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . map . fmt (f) } }
    };
}

impl_86!();