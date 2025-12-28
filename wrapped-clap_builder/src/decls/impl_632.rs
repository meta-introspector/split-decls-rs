macro_rules! deps {
    () => {
        FlatSet!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl < T : PartialEq + Eq > Default for FlatSet < T > { fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_632!();