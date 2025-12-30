// Generated macro for impl_206 (impl)
macro_rules! Depcrate_datetimeimpl_206 {
() => {
// Module: crate::datetime
// Provides: {"impl_206"}
// Dependencies: {}
# [doc = " Convert a `DateTime<Local>` instance into a `DateTime<Utc>` instance."] # [cfg (feature = "clock")] impl From < DateTime < Local > > for DateTime < Utc > { # [doc = " Convert this `DateTime<Local>` instance into a `DateTime<Utc>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`], accounting for the difference in"] # [doc = " timezones."] fn from (src : DateTime < Local >) -> Self { src . with_timezone (& Utc) } }
};
}
