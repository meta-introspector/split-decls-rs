macro_rules! deps {
    () => {
        HasCombination!();
        TupleCombinations!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < I , T > FusedIterator for TupleCombinations < I , T > where I : FusedIterator , T : HasCombination < I > , { }
    };
}

impl_101!()