macro_rules! deps {
    () => {
        SupUnits!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < R : gimli :: Reader > Default for SupUnits < R > { fn default () -> Self { SupUnits { units : Box :: default () , } } }
    };
}

impl_86!();