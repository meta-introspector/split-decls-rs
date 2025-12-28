macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T , const N : usize > Default for SmallVec < T , N > { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_15!()