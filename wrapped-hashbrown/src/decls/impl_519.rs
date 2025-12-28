macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        impl < T , A > FusedIterator for IntoIter < T , A > where A : Allocator { }
    };
}

impl_519!();