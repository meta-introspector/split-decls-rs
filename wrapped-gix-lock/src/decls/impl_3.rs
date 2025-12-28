macro_rules! deps {
    () => {
        Fail!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl From < Duration > for Fail { fn from (value : Duration) -> Self { if value . is_zero () { Fail :: Immediately } else { Fail :: AfterDurationWithBackoff (value) } } }
    };
}

impl_3!();