// Generated macro for impl_25 (impl)
macro_rules! Depcrate_settingsimpl_25 {
() => {
// Module: crate::settings
// Provides: {"impl_25"}
// Dependencies: {}
impl Settings { pub (crate) fn is_set (& self , setting : Setting) -> bool { self . 0 . iter () . any (| s | * s == setting) } pub (crate) fn set (& mut self , setting : Setting) { self . 0 . push (setting) ; } }
};
}
