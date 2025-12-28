macro_rules! deps {
    () => {
        AppExt!();
        Styles!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl super :: AppExt for Styles { }
    };
}

impl_374!()