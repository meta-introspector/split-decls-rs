macro_rules! deps {
    () => {
        Utc!();
        DateTime!();
        Local!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        # [doc = " Convert a `DateTime<Local>` instance into a `DateTime<Utc>` instance."] # [cfg (feature = "clock")] impl From < DateTime < Local > > for DateTime < Utc > { # [doc = " Convert this `DateTime<Local>` instance into a `DateTime<Utc>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`], accounting for the difference in"] # [doc = " timezones."] fn from (src : DateTime < Local >) -> Self { src . with_timezone (& Utc) } }
    };
}

impl_153!();