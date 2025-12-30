// Generated macro for impl_98 (impl)
macro_rules! Depcrate_civil_datetimeimpl_98 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_98"}
// Dependencies: {}
# [doc = " Adds a span of time to a datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_add`]."] impl core :: ops :: Add < Span > for DateTime { type Output = DateTime ; # [inline] fn add (self , rhs : Span) -> DateTime { self . checked_add (rhs) . expect ("adding span to datetime overflowed") } }
};
}
