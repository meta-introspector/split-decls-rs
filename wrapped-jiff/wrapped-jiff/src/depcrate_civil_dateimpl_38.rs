// Generated macro for impl_38 (impl)
macro_rules! Depcrate_civil_dateimpl_38 {
() => {
// Module: crate::civil::date
// Provides: {"impl_38"}
// Dependencies: {}
# [doc = " Adds a span of time to a date."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_add`]."] impl core :: ops :: Add < Span > for Date { type Output = Date ; # [inline] fn add (self , rhs : Span) -> Date { self . checked_add (rhs) . expect ("adding span to date overflowed") } }
};
}
