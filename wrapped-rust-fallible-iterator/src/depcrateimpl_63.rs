// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < I > IntoFallibleIterator for I where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; type IntoFallibleIter = I ; # [inline] fn into_fallible_iter (self) -> I { self } }
};
}
