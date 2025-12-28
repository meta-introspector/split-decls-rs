macro_rules! deps {
    () => {
        Retries!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for Retries { fn default () -> Self { Retries { on_interrupt : 10 , to_create_entire_directory : 5 , on_create_directory_failure : 25 , } } }
    };
}

impl_28!();