// Generated macro for impl_82 (impl)
macro_rules! Depcrate_borrowimpl_82 {
() => {
// Module: crate::borrow
// Provides: {"impl_82"}
// Dependencies: {}
# [stable (feature = "default" , since = "1.11.0")] impl < B : ? Sized > Default for Cow < '_ , B > where B : ToOwned < Owned : Default > , { # [doc = " Creates an owned Cow<'a, B> with the default value for the contained owned value."] fn default () -> Self { Owned (< B as ToOwned > :: Owned :: default ()) } }
};
}
