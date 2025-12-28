macro_rules! deps {
    () => {
        CombinationsWithReplacementGeneric!();
        PoolIndex!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < I , Idx > FusedIterator for CombinationsWithReplacementGeneric < I , Idx > where I : Iterator , I :: Item : Clone , Idx : PoolIndex < I :: Item > , { }
    };
}

impl_195!()