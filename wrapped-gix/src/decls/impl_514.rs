macro_rules! deps {
    () => {
        Permissions!();
        Default!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl Default for Permissions { fn default () -> Self { Permissions :: secure () } }
    };
}

impl_514!();