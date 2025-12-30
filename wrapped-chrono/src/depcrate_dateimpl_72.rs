// Generated macro for impl_72 (impl)
macro_rules! Depcrate_dateimpl_72 {
() => {
// Module: crate::date
// Provides: {"impl_72"}
// Dependencies: {}
impl < Tz : TimeZone > Sub < TimeDelta > for Date < Tz > { type Output = Date < Tz > ; # [inline] fn sub (self , rhs : TimeDelta) -> Date < Tz > { self . checked_sub_signed (rhs) . expect ("`Date - TimeDelta` overflowed") } }
};
}
