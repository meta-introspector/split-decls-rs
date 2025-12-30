// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
# [doc = " An empty reference contains no mutable parts and thus is safe to clone."] impl < 'a , Target : PartialRefTarget > Clone for Ref < 'a , Target > { # [inline (always)] fn clone (& self) -> Self { * self } }
};
}
