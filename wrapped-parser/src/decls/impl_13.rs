macro_rules! deps {
    () => {
        ErrorPositions!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl std :: iter :: FusedIterator for ErrorPositions { }
    };
}

impl_13!()