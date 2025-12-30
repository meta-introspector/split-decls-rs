// Generated macro for impl_1401 (impl)
macro_rules! Depcrate_zonedimpl_1401 {
() => {
// Module: crate::zoned
// Provides: {"impl_1401"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] # [doc = ""] # [doc = " Using this implementation will result in consuming the `Zoned` value. Since"] # [doc = " it is not `Copy`, this will prevent further use. If this is undesirable,"] # [doc = " consider using the trait implementation for `&Zoned`, `Zoned::checked_add`"] # [doc = " or cloning the `Zoned` value."] impl core :: ops :: Add < UnsignedDuration > for Zoned { type Output = Zoned ; # [inline] fn add (self , rhs : UnsignedDuration) -> Zoned { (& self) . add (rhs) } }
};
}
