macro_rules! deps {
    () => {
        CanonicalComposition!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for CanonicalComposition { fn default () -> Self { Self :: new () . static_to_owned () } }
    };
}

impl_19!()