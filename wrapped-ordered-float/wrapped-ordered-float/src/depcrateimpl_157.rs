// Generated macro for impl_157 (impl)
macro_rules! Depcrateimpl_157 {
() => {
// Module: crate
// Provides: {"impl_157"}
// Dependencies: {}
# [doc = " Multiplies a float directly."] # [doc = ""] # [doc = " This returns a `T` and not a `NotNan<T>` because if the multiplied value is NaN, this will be"] # [doc = " NaN"] impl < T : FloatCore > Mul < T > for NotNan < T > { type Output = T ; # [inline] fn mul (self , other : T) -> Self :: Output { self . 0 * other } }
};
}
