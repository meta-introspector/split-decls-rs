// Generated macro for impl_1388 (impl)
macro_rules! Depcrate_zonedimpl_1388 {
() => {
// Module: crate::zoned
// Provides: {"impl_1388"}
// Dependencies: {}
# [doc = " Adds a span of time to a borrowed zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] impl < 'a > core :: ops :: Add < Span > for & 'a Zoned { type Output = Zoned ; # [inline] fn add (self , rhs : Span) -> Zoned { self . checked_add (rhs) . expect ("adding span to zoned datetime overflowed") } }
};
}
