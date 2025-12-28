macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Builder { Builder :: new () } }
    };
}

impl_111!();