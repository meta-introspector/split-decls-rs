macro_rules! deps {
    () => {
        Backoff!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Default for Backoff { fn default () -> Self { Self :: new () } }
    };
}

impl_86!()