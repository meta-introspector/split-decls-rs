macro_rules! deps {
    () => {
        Footer!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Default for Footer { fn default () -> Self { Self :: new () } }
    };
}

impl_27!();