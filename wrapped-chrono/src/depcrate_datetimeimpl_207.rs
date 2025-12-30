// Generated macro for impl_207 (impl)
macro_rules! Depcrate_datetimeimpl_207 {
() => {
// Module: crate::datetime
// Provides: {"impl_207"}
// Dependencies: {}
# [doc = " Convert a `DateTime<Local>` instance into a `DateTime<FixedOffset>` instance."] # [cfg (feature = "clock")] impl From < DateTime < Local > > for DateTime < FixedOffset > { # [doc = " Convert this `DateTime<Local>` instance into a `DateTime<FixedOffset>` instance."] # [doc = ""] # [doc = " Conversion is performed via [`DateTime::with_timezone`]."] fn from (src : DateTime < Local >) -> Self { src . with_timezone (& src . offset () . fix ()) } }
};
}
