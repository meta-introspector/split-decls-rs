macro_rules! deps {
    () => {
        ClaimsValidationRules!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Default for ClaimsValidationRules { fn default () -> Self { Self :: new () } }
    };
}

impl_22!()