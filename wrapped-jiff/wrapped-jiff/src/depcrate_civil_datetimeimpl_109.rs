// Generated macro for impl_109 (impl)
macro_rules! Depcrate_civil_datetimeimpl_109 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_109"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_sub`]."] impl core :: ops :: Sub < UnsignedDuration > for DateTime { type Output = DateTime ; # [inline] fn sub (self , rhs : UnsignedDuration) -> DateTime { self . checked_sub (rhs) . expect ("subtracting unsigned duration from datetime overflowed") } }
};
}
