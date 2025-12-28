macro_rules! deps {
    () => {
        GeneralPurposeConfig!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl Default for GeneralPurposeConfig { # [doc = " Delegates to [`GeneralPurposeConfig::new`]."] fn default () -> Self { Self :: new () } }
    };
}

impl_111!();