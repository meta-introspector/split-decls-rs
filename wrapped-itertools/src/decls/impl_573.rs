macro_rules! deps {
    () => {
        ZipEq!();
    };
}

macro_rules! impl_573 {
    () => {
        deps!();
        impl < I , J > ExactSizeIterator for ZipEq < I , J > where I : ExactSizeIterator , J : ExactSizeIterator , { }
    };
}

impl_573!();