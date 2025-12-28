macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl Default for Select < '_ > { fn default () -> Self { Self :: new () } }
    };
}

impl_181!();