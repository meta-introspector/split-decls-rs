macro_rules! deps {
    () => {
        StartKind!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl Default for StartKind { fn default () -> StartKind { StartKind :: Unanchored } }
    };
}

impl_463!();