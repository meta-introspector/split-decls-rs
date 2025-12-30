// Generated macro for impl_40 (impl)
macro_rules! Depcrate_civil_dateimpl_40 {
() => {
// Module: crate::civil::date
// Provides: {"impl_40"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a date."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_sub`]."] impl core :: ops :: Sub < Span > for Date { type Output = Date ; # [inline] fn sub (self , rhs : Span) -> Date { self . checked_sub (rhs) . expect ("subing span to date overflowed") } }
};
}
