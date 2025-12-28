macro_rules! deps {
    () => {
        Default!();
        Properties!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Default for Properties { fn default () -> Properties { Properties { hidden : true } } }
    };
}

impl_91!()