// Generated macro for impl_204 (impl)
macro_rules! Depcrate_datetimeimpl_204 {
() => {
// Module: crate::datetime
// Provides: {"impl_204"}
// Dependencies: {}
# [doc = " Convert a `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance."] impl From < DateTime < FixedOffset > > for DateTime < Utc > { # [doc = " Convert this `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`], accounting for the timezone"] # [doc = " difference."] fn from (src : DateTime < FixedOffset >) -> Self { src . with_timezone (& Utc) } }
};
}
