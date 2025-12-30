// Generated macro for impl_45 (impl)
macro_rules! Depcrate_civil_dateimpl_45 {
() => {
// Module: crate::civil::date
// Provides: {"impl_45"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a date."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_sub`]."] impl core :: ops :: Sub < SignedDuration > for Date { type Output = Date ; # [inline] fn sub (self , rhs : SignedDuration) -> Date { self . checked_sub (rhs) . expect ("subing signed duration to date overflowed") } }
};
}
