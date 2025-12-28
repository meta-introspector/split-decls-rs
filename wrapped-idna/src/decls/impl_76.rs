macro_rules! deps {
    () => {
        Uts46!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for Uts46 { fn default () -> Self { Self :: new () } }
    };
}

impl_76!();