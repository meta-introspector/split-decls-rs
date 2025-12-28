macro_rules! deps {
    () => {
        DateTime!();
        Local!();
        NaiveDateTime!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        # [cfg (feature = "clock")] impl Default for DateTime < Local > { fn default () -> Self { Local . from_utc_datetime (& NaiveDateTime :: default ()) } }
    };
}

impl_147!()