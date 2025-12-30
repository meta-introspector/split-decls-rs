// Generated macro for impl_205 (impl)
macro_rules! Depcrate_datetimeimpl_205 {
() => {
// Module: crate::datetime
// Provides: {"impl_205"}
// Dependencies: {}
# [doc = " Convert a `DateTime<FixedOffset>` instance into a `DateTime<Local>` instance."] # [cfg (feature = "clock")] impl From < DateTime < FixedOffset > > for DateTime < Local > { # [doc = " Convert this `DateTime<FixedOffset>` instance into a `DateTime<Local>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`]. Returns the equivalent value in local"] # [doc = " time."] fn from (src : DateTime < FixedOffset >) -> Self { src . with_timezone (& Local) } }
};
}
