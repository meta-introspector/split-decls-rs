macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a > FusedIterator for SymmetricDifference < 'a > { }
    };
}

impl_29!()