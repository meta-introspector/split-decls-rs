// Generated macro for impl_202 (impl)
macro_rules! Depcrate_datetimeimpl_202 {
() => {
// Module: crate::datetime
// Provides: {"impl_202"}
// Dependencies: {}
# [doc = " Convert a `DateTime<Utc>` instance into a `DateTime<FixedOffset>` instance."] impl From < DateTime < Utc > > for DateTime < FixedOffset > { # [doc = " Convert this `DateTime<Utc>` instance into a `DateTime<FixedOffset>` instance."] # [doc = ""] # [doc = " Conversion is done via [`DateTime::with_timezone`]. Note that the converted value returned by"] # [doc = " this will be created with a fixed timezone offset of 0."] fn from (src : DateTime < Utc >) -> Self { src . with_timezone (& FixedOffset :: east_opt (0) . unwrap ()) } }
};
}
