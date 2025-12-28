macro_rules! deps {
    () => {
        NonEmptyStringValueParser!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl Default for NonEmptyStringValueParser { fn default () -> Self { Self :: new () } }
    };
}

impl_314!()