macro_rules! deps {
    () => {
        Utc!();
        FixedOffset!();
        DateTime!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [doc = " Convert a `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance."] impl From < DateTime < FixedOffset > > for DateTime < Utc > { # [doc = " Convert this `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`], accounting for the timezone"] # [doc = " difference."] fn from (src : DateTime < FixedOffset >) -> Self { src . with_timezone (& Utc) } }
    };
}

impl_151!()