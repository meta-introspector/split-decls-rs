macro_rules! deps {
    () => {
        DriverChoice!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Default for DriverChoice { fn default () -> Self { DriverChoice :: BuiltIn (Default :: default ()) } }
    };
}

impl_58!();