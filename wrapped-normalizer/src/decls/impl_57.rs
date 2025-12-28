macro_rules! deps {
    () => {
        Uts46MapperBorrowed!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] impl Default for Uts46MapperBorrowed < 'static > { fn default () -> Self { Self :: new () } }
    };
}

impl_57!()