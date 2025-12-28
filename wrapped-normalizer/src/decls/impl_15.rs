macro_rules! deps {
    () => {
        CanonicalCompositionBorrowed!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for CanonicalCompositionBorrowed < 'static > { fn default () -> Self { Self :: new () } }
    };
}

impl_15!();