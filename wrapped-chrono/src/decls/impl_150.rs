macro_rules! deps {
    () => {
        DateTime!();
        Utc!();
        Local!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        # [doc = " Convert a `DateTime<Utc>` instance into a `DateTime<Local>` instance."] # [cfg (feature = "clock")] impl From < DateTime < Utc > > for DateTime < Local > { # [doc = " Convert this `DateTime<Utc>` instance into a `DateTime<Local>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`], accounting for the difference in timezones."] fn from (src : DateTime < Utc >) -> Self { src . with_timezone (& Local) } }
    };
}

impl_150!()