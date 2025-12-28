macro_rules! deps {
    () => {
        Local!();
        DateTime!();
        FixedOffset!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        # [doc = " Convert a `DateTime<FixedOffset>` instance into a `DateTime<Local>` instance."] # [cfg (feature = "clock")] impl From < DateTime < FixedOffset > > for DateTime < Local > { # [doc = " Convert this `DateTime<FixedOffset>` instance into a `DateTime<Local>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`]. Returns the equivalent value in local"] # [doc = " time."] fn from (src : DateTime < FixedOffset >) -> Self { src . with_timezone (& Local) } }
    };
}

impl_152!();