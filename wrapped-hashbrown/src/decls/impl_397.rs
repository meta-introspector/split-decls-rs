macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < T , S , A > Eq for HashSet < T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { }
    };
}

impl_397!()