macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T > Hash for RevSlice < T > where T : Hash , { fn hash < H : Hasher > (& self , h : & mut H) { self . len () . hash (h) ; for elt in self { elt . hash (h) } } }
    };
}

impl_53!()