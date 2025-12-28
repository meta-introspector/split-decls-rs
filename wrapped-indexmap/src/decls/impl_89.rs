macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < T , S > Eq for IndexSet < T , S > where T : Eq + Hash , S : BuildHasher , { }
    };
}

impl_89!();