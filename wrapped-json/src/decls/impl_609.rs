macro_rules! deps {
    () => {
        RawValue!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl Default for Box < RawValue > { fn default () -> Self { RawValue :: NULL . to_owned () } }
    };
}

impl_609!()