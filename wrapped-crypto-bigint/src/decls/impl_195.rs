macro_rules! deps {
    () => {
        Constants!();
        NonZero!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T > Default for NonZero < T > where T : Constants , { fn default () -> Self { Self (T :: ONE) } }
    };
}

impl_195!();