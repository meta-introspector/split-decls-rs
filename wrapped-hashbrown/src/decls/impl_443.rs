macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl < T , S , A > FusedIterator for Intersection < '_ , T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { }
    };
}

impl_443!();