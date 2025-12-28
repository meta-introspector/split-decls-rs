macro_rules! deps {
    () => {
        Stylesheet!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Default for Stylesheet { fn default () -> Self { Self :: plain () } }
    };
}

impl_76!();