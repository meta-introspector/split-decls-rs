macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < T , S > Hash for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { for e in self { e . hash (state) ; } } }
    };
}

impl_138!();