macro_rules! deps {
    () => {
        CanonicalCombiningClassMap!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for CanonicalCombiningClassMap { fn default () -> Self { Self :: new () . static_to_owned () } }
    };
}

impl_34!()