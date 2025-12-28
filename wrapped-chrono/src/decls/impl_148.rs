macro_rules! deps {
    () => {
        FixedOffset!();
        NaiveDateTime!();
        DateTime!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl Default for DateTime < FixedOffset > { fn default () -> Self { FixedOffset :: west_opt (0) . unwrap () . from_utc_datetime (& NaiveDateTime :: default ()) } }
    };
}

impl_148!()