macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl Float for f32 { }
    };
}

impl_371!();