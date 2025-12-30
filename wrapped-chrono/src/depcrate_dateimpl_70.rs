// Generated macro for impl_70 (impl)
macro_rules! Depcrate_dateimpl_70 {
() => {
// Module: crate::date
// Provides: {"impl_70"}
// Dependencies: {}
impl < Tz : TimeZone > Add < TimeDelta > for Date < Tz > { type Output = Date < Tz > ; # [inline] fn add (self , rhs : TimeDelta) -> Date < Tz > { self . checked_add_signed (rhs) . expect ("`Date + TimeDelta` overflowed") } }
};
}
