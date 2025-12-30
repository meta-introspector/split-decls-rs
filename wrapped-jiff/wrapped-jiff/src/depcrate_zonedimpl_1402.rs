// Generated macro for impl_1402 (impl)
macro_rules! Depcrate_zonedimpl_1402 {
() => {
// Module: crate::zoned
// Provides: {"impl_1402"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a borrowed zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] impl < 'a > core :: ops :: Add < UnsignedDuration > for & 'a Zoned { type Output = Zoned ; # [inline] fn add (self , rhs : UnsignedDuration) -> Zoned { self . checked_add (rhs) . expect ("adding unsigned duration to zoned datetime overflowed") } }
};
}
