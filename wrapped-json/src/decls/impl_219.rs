macro_rules! deps {
    () => {
        PrettyFormatter!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < 'a > Default for PrettyFormatter < 'a > { fn default () -> Self { PrettyFormatter :: new () } }
    };
}

impl_219!();