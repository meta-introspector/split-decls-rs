macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl Default for RootDatabase { fn default () -> RootDatabase { RootDatabase :: new (None) } }
    };
}

impl_253!();