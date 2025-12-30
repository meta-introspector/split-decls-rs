// Generated macro for impl_1390 (impl)
macro_rules! Depcrate_zonedimpl_1390 {
() => {
// Module: crate::zoned
// Provides: {"impl_1390"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a zoned datetime."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] # [doc = ""] # [doc = " Using this implementation will result in consuming the `Zoned` value. Since"] # [doc = " it is not `Copy`, this will prevent further use. If this is undesirable,"] # [doc = " consider using the trait implementation for `&Zoned`, `Zoned::checked_sub`"] # [doc = " or cloning the `Zoned` value."] impl < 'a > core :: ops :: Sub < Span > for Zoned { type Output = Zoned ; # [inline] fn sub (self , rhs : Span) -> Zoned { (& self) . sub (rhs) } }
};
}
