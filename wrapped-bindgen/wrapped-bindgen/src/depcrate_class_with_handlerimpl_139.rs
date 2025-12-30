// Generated macro for impl_139 (impl)
macro_rules! Depcrate_class_with_handlerimpl_139 {
() => {
// Module: crate::class_with_handler
// Provides: {"impl_139"}
// Dependencies: {}
impl IClosable { pub fn Close (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
