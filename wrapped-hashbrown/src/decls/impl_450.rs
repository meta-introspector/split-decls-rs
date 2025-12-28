macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl < T , S , A > FusedIterator for SymmetricDifference < '_ , T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { }
    };
}

impl_450!()