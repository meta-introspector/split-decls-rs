macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl < T , const N : usize > Default for Queue < T , N > { fn default () -> Self { Self :: new () } }
    };
}

impl_458!();