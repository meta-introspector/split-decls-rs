macro_rules! deps {
    () => {
        Deque!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T , const N : usize > Default for Deque < T , N > { fn default () -> Self { Self :: new () } }
    };
}

impl_36!();