macro_rules! deps {
    () => {
        StatusOptions!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl Default for StatusOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_749!()