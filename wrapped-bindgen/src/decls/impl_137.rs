macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl Default for TokenStream { fn default () -> Self { Self :: new () } }
    };
}

impl_137!()