// Generated macro for impl_71 (impl)
macro_rules! Depcrate_dateimpl_71 {
() => {
// Module: crate::date
// Provides: {"impl_71"}
// Dependencies: {}
impl < Tz : TimeZone > AddAssign < TimeDelta > for Date < Tz > { # [inline] fn add_assign (& mut self , rhs : TimeDelta) { self . date = self . date . checked_add_signed (rhs) . expect ("`Date + TimeDelta` overflowed") ; } }
};
}
