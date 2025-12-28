macro_rules! deps {
    () => {
        Line!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Default for Line < '_ > { fn default () -> Self { Self :: new (& []) } }
    };
}

impl_17!();