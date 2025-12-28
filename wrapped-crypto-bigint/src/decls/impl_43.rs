macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > Default for Checked < T > where T : Default , { fn default () -> Self { Self :: new (T :: default ()) } }
    };
}

impl_43!();