macro_rules! deps {
    () => {
        GroupInfoPatternNames!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for GroupInfoPatternNames < 'a > { }
    };
}

impl_632!()