macro_rules! deps {
    () => {
        Probe!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl Default for Probe { # [inline] fn default () -> Probe { Probe :: new () } }
    };
}

impl_120!();