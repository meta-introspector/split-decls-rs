macro_rules! deps {
    () => {
        ConstBuffer!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Default for ConstBuffer { fn default () -> Self { Self :: new () } }
    };
}

impl_77!()