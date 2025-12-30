// Generated macro for impl_160 (impl)
macro_rules! Depcrateimpl_160 {
() => {
// Module: crate
// Provides: {"impl_160"}
// Dependencies: {}
# [doc = " Divides a float directly."] # [doc = ""] # [doc = " This returns a `T` and not a `NotNan<T>` because if the divided-by value is NaN, this will be"] # [doc = " NaN"] impl < T : FloatCore > Div < T > for NotNan < T > { type Output = T ; # [inline] fn div (self , other : T) -> Self :: Output { self . 0 / other } }
};
}
