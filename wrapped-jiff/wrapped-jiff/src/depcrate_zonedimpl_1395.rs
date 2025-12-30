// Generated macro for impl_1395 (impl)
macro_rules! Depcrate_zonedimpl_1395 {
() => {
// Module: crate::zoned
// Provides: {"impl_1395"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] # [doc = ""] # [doc = " Using this implementation will result in consuming the `Zoned` value. Since"] # [doc = " it is not `Copy`, this will prevent further use. If this is undesirable,"] # [doc = " consider using the trait implementation for `&Zoned`, `Zoned::checked_add`"] # [doc = " or cloning the `Zoned` value."] impl core :: ops :: Add < SignedDuration > for Zoned { type Output = Zoned ; # [inline] fn add (self , rhs : SignedDuration) -> Zoned { (& self) . add (rhs) } }
};
}
