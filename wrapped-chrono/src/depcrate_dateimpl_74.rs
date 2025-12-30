// Generated macro for impl_74 (impl)
macro_rules! Depcrate_dateimpl_74 {
() => {
// Module: crate::date
// Provides: {"impl_74"}
// Dependencies: {}
impl < Tz : TimeZone > Sub < Date < Tz > > for Date < Tz > { type Output = TimeDelta ; # [inline] fn sub (self , rhs : Date < Tz >) -> TimeDelta { self . signed_duration_since (rhs) } }
};
}
