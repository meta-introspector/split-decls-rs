macro_rules! deps {
    () => {
        Styles!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl Default for Styles { fn default () -> Self { Self :: styled () } }
    };
}

impl_375!();