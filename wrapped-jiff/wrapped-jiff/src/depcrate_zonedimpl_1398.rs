// Generated macro for impl_1398 (impl)
macro_rules! Depcrate_zonedimpl_1398 {
() => {
// Module: crate::zoned
// Provides: {"impl_1398"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] # [doc = ""] # [doc = " Using this implementation will result in consuming the `Zoned` value. Since"] # [doc = " it is not `Copy`, this will prevent further use. If this is undesirable,"] # [doc = " consider using the trait implementation for `&Zoned`, `Zoned::checked_sub`"] # [doc = " or cloning the `Zoned` value."] impl core :: ops :: Sub < SignedDuration > for Zoned { type Output = Zoned ; # [inline] fn sub (self , rhs : SignedDuration) -> Zoned { (& self) . sub (rhs) } }
};
}
