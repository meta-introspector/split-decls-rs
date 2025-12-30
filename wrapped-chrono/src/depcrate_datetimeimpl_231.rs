// Generated macro for impl_231 (impl)
macro_rules! Depcrate_datetimeimpl_231 {
() => {
// Module: crate::datetime
// Provides: {"impl_231"}
// Dependencies: {}
impl < Tz : TimeZone > Sub < DateTime < Tz > > for DateTime < Tz > { type Output = TimeDelta ; # [inline] fn sub (self , rhs : DateTime < Tz >) -> TimeDelta { self . signed_duration_since (rhs) } }
};
}
