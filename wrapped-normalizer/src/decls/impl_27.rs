macro_rules! deps {
    () => {
        CanonicalDecomposition!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for CanonicalDecomposition { fn default () -> Self { Self :: new () . static_to_owned () } }
    };
}

impl_27!()