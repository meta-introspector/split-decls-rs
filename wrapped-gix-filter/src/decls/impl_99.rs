macro_rules! deps {
    () => {
        Pipeline!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Default for Pipeline { fn default () -> Self { Pipeline :: new (Default :: default () , Default :: default ()) } }
    };
}

impl_99!()