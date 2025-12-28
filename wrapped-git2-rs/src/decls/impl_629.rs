macro_rules! deps {
    () => {
        ReflogIter!();
    };
}

macro_rules! impl_629 {
    () => {
        deps!();
        impl < 'reflog > ExactSizeIterator for ReflogIter < 'reflog > { }
    };
}

impl_629!();