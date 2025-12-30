// Generated macro for impl_100 (impl)
macro_rules! Depcrate_civil_datetimeimpl_100 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_100"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_sub`]."] impl core :: ops :: Sub < Span > for DateTime { type Output = DateTime ; # [inline] fn sub (self , rhs : Span) -> DateTime { self . checked_sub (rhs) . expect ("subtracting span from datetime overflowed") } }
};
}
