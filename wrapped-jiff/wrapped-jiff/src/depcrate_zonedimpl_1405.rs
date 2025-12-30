// Generated macro for impl_1405 (impl)
macro_rules! Depcrate_zonedimpl_1405 {
() => {
// Module: crate::zoned
// Provides: {"impl_1405"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a borrowed zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] impl < 'a > core :: ops :: Sub < UnsignedDuration > for & 'a Zoned { type Output = Zoned ; # [inline] fn sub (self , rhs : UnsignedDuration) -> Zoned { self . checked_sub (rhs) . expect ("subtracting unsigned duration from zoned datetime overflowed" ,) } }
};
}
