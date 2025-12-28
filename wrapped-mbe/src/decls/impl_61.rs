macro_rules! deps {
    () => {
        ValueResult!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : Default , E > Default for ValueResult < T , E > { fn default () -> Self { Self { value : Default :: default () , err : Default :: default () } } }
    };
}

impl_61!();