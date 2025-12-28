macro_rules! deps {
    () => {
        Dependency!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Default for Dependency { fn default () -> Self { Dependency :: Version (String :: new ()) } }
    };
}

impl_6!()