macro_rules! deps {
    () => {
        One!();
        Limb!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl One for Limb { # [inline (always)] fn one () -> Self { Self :: ONE } }
    };
}

impl_162!()