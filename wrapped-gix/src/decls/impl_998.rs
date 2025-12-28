macro_rules! deps {
    () => {
        Default!();
        Submodule!();
    };
}

macro_rules! impl_998 {
    () => {
        deps!();
        impl Default for Submodule { fn default () -> Self { Submodule :: AsConfigured { check_dirty : false } } }
    };
}

impl_998!()