// Generated macro for impl_65 (impl)
macro_rules! Depcrate_arcimpl_65 {
() => {
// Module: crate::arc
// Provides: {"impl_65"}
// Dependencies: {}
impl < T > From < T > for Arc < T > { # [doc = " Converts a `T` into an `Arc<T>`"] # [doc = ""] # [doc = " The conversion moves the value into a"] # [doc = " newly allocated `Arc`. It is equivalent to"] # [doc = " calling `Arc::new(t)`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let x = 5;"] # [doc = " let arc = Arc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Arc::from(x), arc);"] # [doc = " ```"] fn from (t : T) -> Self { Self :: new (t) } }
};
}
