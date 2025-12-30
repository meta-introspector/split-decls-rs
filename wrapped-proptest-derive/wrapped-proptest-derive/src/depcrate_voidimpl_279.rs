// Generated macro for impl_279 (impl)
macro_rules! Depcrate_voidimpl_279 {
() => {
// Module: crate::void
// Provides: {"impl_279"}
// Dependencies: {}
impl < P > IsUninhabited for syn :: punctuated :: Punctuated < syn :: Variant , P > { fn is_uninhabited (& self) -> bool { self . iter () . all (IsUninhabited :: is_uninhabited) } }
};
}
