macro_rules! deps {
    () => {
        MaybeUndefined!();
    };
}

macro_rules! impl_785 {
    () => {
        deps!();
        impl < T > Default for MaybeUndefined < T > { fn default () -> Self { Self :: Undefined } }
    };
}

impl_785!()