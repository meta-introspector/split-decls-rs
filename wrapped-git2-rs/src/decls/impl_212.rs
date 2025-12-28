macro_rules! deps {
    () => {
        BlameIter!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'blame > ExactSizeIterator for BlameIter < 'blame > { }
    };
}

impl_212!()