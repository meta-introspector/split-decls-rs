macro_rules! deps {
    () => {
        Refspecs!();
    };
}

macro_rules! impl_650 {
    () => {
        deps!();
        impl < 'repo > FusedIterator for Refspecs < 'repo > { }
    };
}

impl_650!();