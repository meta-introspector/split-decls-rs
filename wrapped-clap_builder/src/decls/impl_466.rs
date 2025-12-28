macro_rules! deps {
    () => {
        Occurrences!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for Occurrences < T > { }
    };
}

impl_466!()