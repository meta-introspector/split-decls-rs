// Generated macro for impl_280 (impl)
macro_rules! Depcrate_voidimpl_280 {
() => {
// Module: crate::void
// Provides: {"impl_280"}
// Dependencies: {}
impl < 'a > IsUninhabited for & 'a [syn :: Variant] { fn is_uninhabited (& self) -> bool { self . iter () . all (IsUninhabited :: is_uninhabited) } }
};
}
