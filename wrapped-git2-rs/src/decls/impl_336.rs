macro_rules! deps {
    () => {
        DiffOptions!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl Default for DiffOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_336!()