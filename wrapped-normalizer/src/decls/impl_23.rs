macro_rules! deps {
    () => {
        CanonicalDecompositionBorrowed!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for CanonicalDecompositionBorrowed < 'static > { fn default () -> Self { Self :: new () } }
    };
}

impl_23!()