macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < K > ExactSizeIterator for IntoIter < K > { }
    };
}

impl_164!();