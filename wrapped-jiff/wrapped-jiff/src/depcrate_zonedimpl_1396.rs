// Generated macro for impl_1396 (impl)
macro_rules! Depcrate_zonedimpl_1396 {
() => {
// Module: crate::zoned
// Provides: {"impl_1396"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a borrowed zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] impl < 'a > core :: ops :: Add < SignedDuration > for & 'a Zoned { type Output = Zoned ; # [inline] fn add (self , rhs : SignedDuration) -> Zoned { self . checked_add (rhs) . expect ("adding signed duration to zoned datetime overflowed") } }
};
}
