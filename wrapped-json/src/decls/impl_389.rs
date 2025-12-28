macro_rules! deps {
    () => {
        Bigint!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl Default for Bigint { fn default () -> Self { Bigint { data : Vec :: with_capacity (20) , } } }
    };
}

impl_389!();