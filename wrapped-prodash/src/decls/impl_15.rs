macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { initial_capacity : 100 , message_buffer_capacity : 20 , } } }
    };
}

impl_15!();