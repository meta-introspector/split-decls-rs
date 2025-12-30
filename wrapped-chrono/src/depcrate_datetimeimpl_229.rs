// Generated macro for impl_229 (impl)
macro_rules! Depcrate_datetimeimpl_229 {
() => {
// Module: crate::datetime
// Provides: {"impl_229"}
// Dependencies: {}
# [doc = " Subtract `FixedOffset` from the datetime value of `DateTime` (offset remains unchanged)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] impl < Tz : TimeZone > Sub < FixedOffset > for DateTime < Tz > { type Output = DateTime < Tz > ; # [inline] fn sub (mut self , rhs : FixedOffset) -> DateTime < Tz > { self . datetime = self . naive_utc () . checked_sub_offset (rhs) . expect ("`DateTime - FixedOffset` overflowed") ; self } }
};
}
