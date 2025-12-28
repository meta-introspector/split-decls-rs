macro_rules! deps {
    () => {
        Default!();
        Attributes!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl Default for Attributes { fn default () -> Self { Self :: all () } }
    };
}

impl_509!();