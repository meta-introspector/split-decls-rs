// Generated macro for impl_203 (impl)
macro_rules! Depcrate_datetimeimpl_203 {
() => {
// Module: crate::datetime
// Provides: {"impl_203"}
// Dependencies: {}
# [doc = " Convert a `DateTime<Utc>` instance into a `DateTime<Local>` instance."] # [cfg (feature = "clock")] impl From < DateTime < Utc > > for DateTime < Local > { # [doc = " Convert this `DateTime<Utc>` instance into a `DateTime<Local>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`], accounting for the difference in timezones."] fn from (src : DateTime < Utc >) -> Self { src . with_timezone (& Local) } }
};
}
