// Generated macro for impl_232 (impl)
macro_rules! Depcrate_datetimeimpl_232 {
() => {
// Module: crate::datetime
// Provides: {"impl_232"}
// Dependencies: {}
impl < Tz : TimeZone > Sub < & DateTime < Tz > > for DateTime < Tz > { type Output = TimeDelta ; # [inline] fn sub (self , rhs : & DateTime < Tz >) -> TimeDelta { self . signed_duration_since (rhs) } }
};
}
