// Generated macro for impl_73 (impl)
macro_rules! Depcrate_dateimpl_73 {
() => {
// Module: crate::date
// Provides: {"impl_73"}
// Dependencies: {}
impl < Tz : TimeZone > SubAssign < TimeDelta > for Date < Tz > { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { self . date = self . date . checked_sub_signed (rhs) . expect ("`Date - TimeDelta` overflowed") ; } }
};
}
