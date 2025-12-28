macro_rules! deps {
    () => {
        Zero!();
        NotZero!();
    };
}

macro_rules! impl_972 {
    () => {
        deps!();
        impl < T : Zero > Default for NotZero < T > { fn default () -> Self { NotZero (T :: zero ()) } }
    };
}

impl_972!();