macro_rules! deps {
    () => {
        BlameIter!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'blame > FusedIterator for BlameIter < 'blame > { }
    };
}

impl_211!()