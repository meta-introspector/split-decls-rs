macro_rules! deps {
    () => {
        Uts46Mapper!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for Uts46Mapper { fn default () -> Self { Self :: new () . static_to_owned () } }
    };
}

impl_61!();