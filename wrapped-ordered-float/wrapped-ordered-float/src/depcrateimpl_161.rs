// Generated macro for impl_161 (impl)
macro_rules! Depcrateimpl_161 {
() => {
// Module: crate
// Provides: {"impl_161"}
// Dependencies: {}
# [doc = " Calculates `%` with a float directly."] # [doc = ""] # [doc = " This returns a `T` and not a `NotNan<T>` because if the RHS is NaN, this will be NaN"] impl < T : FloatCore > Rem < T > for NotNan < T > { type Output = T ; # [inline] fn rem (self , other : T) -> Self :: Output { self . 0 % other } }
};
}
