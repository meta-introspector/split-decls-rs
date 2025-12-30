// Generated macro for impl_49 (impl)
macro_rules! Depcrate_civil_dateimpl_49 {
() => {
// Module: crate::civil::date
// Provides: {"impl_49"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a date."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_sub`]."] impl core :: ops :: Sub < UnsignedDuration > for Date { type Output = Date ; # [inline] fn sub (self , rhs : UnsignedDuration) -> Date { self . checked_sub (rhs) . expect ("subing unsigned duration to date overflowed") } }
};
}
