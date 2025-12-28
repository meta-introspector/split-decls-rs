macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'a > FusedIterator for SymmetricDifference < 'a > { }
    };
}

impl_98!();