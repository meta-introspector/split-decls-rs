macro_rules! deps {
    () => {
        ErrorPositions!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl std :: iter :: FusedIterator for ErrorPositions { }
    };
}

impl_144!()