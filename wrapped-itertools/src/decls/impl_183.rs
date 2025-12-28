macro_rules! deps {
    () => {
        PoolIndex!();
        CombinationsGeneric!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < I , Idx > FusedIterator for CombinationsGeneric < I , Idx > where I : Iterator , I :: Item : Clone , Idx : PoolIndex < I :: Item > , { }
    };
}

impl_183!()