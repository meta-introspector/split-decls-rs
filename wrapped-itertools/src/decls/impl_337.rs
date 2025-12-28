macro_rules! deps {
    () => {
        KMergeBy!();
        KMergePredicate!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < I , F > FusedIterator for KMergeBy < I , F > where I : Iterator , F : KMergePredicate < I :: Item > , { }
    };
}

impl_337!();