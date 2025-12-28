macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl < T , const N : usize > Default for Queue < T , N > { fn default () -> Self { # [allow (deprecated)] Self :: new () } }
    };
}

impl_405!();