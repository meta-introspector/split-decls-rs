macro_rules! deps {
    () => {
        FixedOffset!();
        DateTime!();
        Local!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        # [doc = " Convert a `DateTime<Local>` instance into a `DateTime<FixedOffset>` instance."] # [cfg (feature = "clock")] impl From < DateTime < Local > > for DateTime < FixedOffset > { # [doc = " Convert this `DateTime<Local>` instance into a `DateTime<FixedOffset>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`]."] fn from (src : DateTime < Local >) -> Self { src . with_timezone (& src . offset () . fix ()) } }
    };
}

impl_154!()