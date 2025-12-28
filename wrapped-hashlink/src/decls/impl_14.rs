macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < K : Hash + Eq , V : Hash , S : BuildHasher > Hash for LinkedHashMap < K , V , S > { # [inline] fn hash < H : Hasher > (& self , h : & mut H) { for e in self . iter () { e . hash (h) ; } } }
    };
}

impl_14!();