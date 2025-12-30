// Generated macro for impl_1391 (impl)
macro_rules! Depcrate_zonedimpl_1391 {
() => {
// Module: crate::zoned
// Provides: {"impl_1391"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a borrowed zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] impl < 'a > core :: ops :: Sub < Span > for & 'a Zoned { type Output = Zoned ; # [inline] fn sub (self , rhs : Span) -> Zoned { self . checked_sub (rhs) . expect ("subtracting span from zoned datetime overflowed") } }
};
}
