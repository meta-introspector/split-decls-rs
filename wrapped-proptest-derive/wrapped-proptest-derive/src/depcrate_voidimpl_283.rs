// Generated macro for impl_283 (impl)
macro_rules! Depcrate_voidimpl_283 {
() => {
// Module: crate::void
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'a > IsUninhabited for & 'a [syn :: Field] { fn is_uninhabited (& self) -> bool { self . iter () . any (syn :: Field :: is_uninhabited) } }
};
}
