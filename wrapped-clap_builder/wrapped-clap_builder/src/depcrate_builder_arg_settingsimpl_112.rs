// Generated macro for impl_112 (impl)
macro_rules! Depcrate_builder_arg_settingsimpl_112 {
() => {
// Module: crate::builder::arg_settings
// Provides: {"impl_112"}
// Dependencies: {}
impl ArgFlags { pub (crate) fn set (& mut self , setting : ArgSettings) { self . 0 |= setting . bit () ; } pub (crate) fn unset (& mut self , setting : ArgSettings) { self . 0 &= ! setting . bit () ; } pub (crate) fn is_set (& self , setting : ArgSettings) -> bool { self . 0 & setting . bit () != 0 } pub (crate) fn insert (& mut self , other : Self) { self . 0 |= other . 0 ; } }
};
}
