macro_rules! deps {
    () => {
        ContextSize!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl Default for ContextSize { fn default () -> Self { ContextSize :: symmetrical (3) } }
    };
}

impl_127!()