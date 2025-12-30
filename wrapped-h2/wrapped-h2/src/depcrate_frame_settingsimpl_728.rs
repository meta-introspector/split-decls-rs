// Generated macro for impl_728 (impl)
macro_rules! Depcrate_frame_settingsimpl_728 {
() => {
// Module: crate::frame::settings
// Provides: {"impl_728"}
// Dependencies: {}
impl SettingsFlags { pub fn empty () -> SettingsFlags { SettingsFlags (0) } pub fn load (bits : u8) -> SettingsFlags { SettingsFlags (bits & ALL) } pub fn ack () -> SettingsFlags { SettingsFlags (ACK) } pub fn is_ack (& self) -> bool { self . 0 & ACK == ACK } }
};
}
