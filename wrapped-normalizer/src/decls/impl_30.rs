macro_rules! deps {
    () => {
        CanonicalCombiningClassMapBorrowed!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for CanonicalCombiningClassMapBorrowed < 'static > { fn default () -> Self { Self :: new () } }
    };
}

impl_30!();