macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        impl < T , S , A > FusedIterator for Union < '_ , T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { }
    };
}

impl_453!();