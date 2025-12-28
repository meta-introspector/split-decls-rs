macro_rules! deps {
    () => {
        Permutations!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < I > FusedIterator for Permutations < I > where I : Iterator , I :: Item : Clone , { }
    };
}

impl_432!();