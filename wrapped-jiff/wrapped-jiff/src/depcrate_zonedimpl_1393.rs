// Generated macro for impl_1393 (impl)
macro_rules! Depcrate_zonedimpl_1393 {
() => {
// Module: crate::zoned
// Provides: {"impl_1393"}
// Dependencies: {}
# [doc = " Computes the span of time between two zoned datetimes."] # [doc = ""] # [doc = " This will return a negative span when the zoned datetime being subtracted"] # [doc = " is greater."] # [doc = ""] # [doc = " Since this uses the default configuration for calculating a span between"] # [doc = " two zoned datetimes (no rounding and largest units is hours), this will"] # [doc = " never panic or fail in any way. It is guaranteed that the largest non-zero"] # [doc = " unit in the `Span` returned will be hours."] # [doc = ""] # [doc = " To configure the largest unit or enable rounding, use [`Zoned::since`]."] # [doc = ""] # [doc = " Using this implementation will result in consuming the `Zoned` value. Since"] # [doc = " it is not `Copy`, this will prevent further use. If this is undesirable,"] # [doc = " consider using the trait implementation for `&Zoned`, `Zoned::since`,"] # [doc = " `Zoned::until` or cloning the `Zoned` value."] impl core :: ops :: Sub for Zoned { type Output = Span ; # [inline] fn sub (self , rhs : Zoned) -> Span { (& self) . sub (& rhs) } }
};
}
