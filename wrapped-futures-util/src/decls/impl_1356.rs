macro_rules! deps {
    () => {
        OkFn!();
    };
}

macro_rules! impl_1356 {
    () => {
        deps!();
        impl < E > Default for OkFn < E > { fn default () -> Self { Self (PhantomData) } }
    };
}

impl_1356!()