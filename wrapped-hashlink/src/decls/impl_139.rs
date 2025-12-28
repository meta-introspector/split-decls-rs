macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < T , S > Eq for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { }
    };
}

impl_139!();