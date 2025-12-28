macro_rules! deps {
    () => {
        DateTime!();
        Utc!();
        NaiveDateTime!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Default for DateTime < Utc > { fn default () -> Self { Utc . from_utc_datetime (& NaiveDateTime :: default ()) } }
    };
}

impl_146!()