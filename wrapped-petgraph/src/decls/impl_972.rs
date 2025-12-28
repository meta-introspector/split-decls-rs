macro_rules! deps {
    () => {
        NotZero!();
        Zero!();
    };
}

macro_rules! impl_972 {
    () => {
        deps!();
        impl < T : Zero > Default for NotZero < T > { fn default () -> Self { NotZero (T :: zero ()) } }
    };
}

impl_972!()