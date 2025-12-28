macro_rules! deps {
    () => {
        IntoFn!();
    };
}

macro_rules! impl_1400 {
    () => {
        deps!();
        impl < T > Default for IntoFn < T > { fn default () -> Self { Self (PhantomData) } }
    };
}

impl_1400!()