// Generated macro for impl_57 (impl)
macro_rules! Depcrate_builder_app_settingsimpl_57 {
() => {
// Module: crate::builder::app_settings
// Provides: {"impl_57"}
// Dependencies: {}
impl AppFlags { pub (crate) fn set (& mut self , setting : AppSettings) { self . 0 |= setting . bit () ; } pub (crate) fn unset (& mut self , setting : AppSettings) { self . 0 &= ! setting . bit () ; } pub (crate) fn is_set (& self , setting : AppSettings) -> bool { self . 0 & setting . bit () != 0 } pub (crate) fn insert (& mut self , other : Self) { self . 0 |= other . 0 ; } }
};
}
