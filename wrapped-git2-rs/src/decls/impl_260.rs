macro_rules! deps {
    () => {
        ParentIds!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < 'commit > ExactSizeIterator for ParentIds < 'commit > { }
    };
}

impl_260!()