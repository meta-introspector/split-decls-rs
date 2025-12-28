macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < T , S , A > FusedIterator for Difference < '_ , T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { }
    };
}

impl_446!()