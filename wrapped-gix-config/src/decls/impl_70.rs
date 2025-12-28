macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Default for Metadata { fn default () -> Self { Metadata :: api () } }
    };
}

impl_70!()