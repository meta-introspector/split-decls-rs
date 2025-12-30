// Generated macro for impl_153 (impl)
macro_rules! Depcrateimpl_153 {
() => {
// Module: crate
// Provides: {"impl_153"}
// Dependencies: {}
# [doc = " Adds a float directly."] # [doc = ""] # [doc = " This returns a `T` and not a `NotNan<T>` because if the added value is NaN, this will be NaN"] impl < T : FloatCore > Add < T > for NotNan < T > { type Output = T ; # [inline] fn add (self , other : T) -> Self :: Output { self . 0 + other } }
};
}
