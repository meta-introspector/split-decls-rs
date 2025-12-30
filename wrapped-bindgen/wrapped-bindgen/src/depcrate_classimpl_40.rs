// Generated macro for impl_40 (impl)
macro_rules! Depcrate_classimpl_40 {
() => {
// Module: crate::class
// Provides: {"impl_40"}
// Dependencies: {}
impl IClosable { pub fn Close (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
