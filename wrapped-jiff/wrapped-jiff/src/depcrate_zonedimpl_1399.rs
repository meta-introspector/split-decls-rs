// Generated macro for impl_1399 (impl)
macro_rules! Depcrate_zonedimpl_1399 {
() => {
// Module: crate::zoned
// Provides: {"impl_1399"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a borrowed zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] impl < 'a > core :: ops :: Sub < SignedDuration > for & 'a Zoned { type Output = Zoned ; # [inline] fn sub (self , rhs : SignedDuration) -> Zoned { self . checked_sub (rhs) . expect ("subtracting signed duration from zoned datetime overflowed" ,) } }
};
}
