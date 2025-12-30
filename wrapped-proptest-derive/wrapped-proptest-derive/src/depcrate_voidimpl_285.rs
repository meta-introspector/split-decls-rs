// Generated macro for impl_285 (impl)
macro_rules! Depcrate_voidimpl_285 {
() => {
// Module: crate::void
// Provides: {"impl_285"}
// Dependencies: {}
impl IsUninhabited for syn :: Type { fn is_uninhabited (& self) -> bool { let mut uninhabited = Uninhabited (false) ; visit :: visit_type (& mut uninhabited , & self) ; uninhabited . 0 } }
};
}
