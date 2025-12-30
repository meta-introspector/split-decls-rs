// Generated macro for impl_156 (impl)
macro_rules! Depcrateimpl_156 {
() => {
// Module: crate
// Provides: {"impl_156"}
// Dependencies: {}
# [doc = " Subtracts a float directly."] # [doc = ""] # [doc = " This returns a `T` and not a `NotNan<T>` because if the substracted value is NaN, this will be"] # [doc = " NaN"] impl < T : FloatCore > Sub < T > for NotNan < T > { type Output = T ; # [inline] fn sub (self , other : T) -> Self :: Output { self . 0 - other } }
};
}
