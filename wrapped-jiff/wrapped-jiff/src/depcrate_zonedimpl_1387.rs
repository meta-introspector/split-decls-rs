// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_zonedimpl_1387 {
() => {
// Module: crate::zoned
// Provides: {"impl_1387"}
// Dependencies: {}
# [doc = " Adds a span of time to a zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] # [doc = ""] # [doc = " Using this implementation will result in consuming the `Zoned` value. Since"] # [doc = " it is not `Copy`, this will prevent further use. If this is undesirable,"] # [doc = " consider using the trait implementation for `&Zoned`, `Zoned::checked_add`"] # [doc = " or cloning the `Zoned` value."] impl < 'a > core :: ops :: Add < Span > for Zoned { type Output = Zoned ; # [inline] fn add (self , rhs : Span) -> Zoned { (& self) . add (rhs) } }
};
}
