// Generated macro for impl_335 (impl)
macro_rules! Depcrate_settingsimpl_335 {
() => {
// Module: crate::settings
// Provides: {"impl_335"}
// Dependencies: {}
impl Drop for SettingsBindDropGuard { fn drop (& mut self) { CURRENT_SETTINGS . with (| x | { x . borrow_mut () . inner = self . 0 . take () . unwrap () ; }) } }
};
}
