// Generated macro for impl_50 (impl)
macro_rules! Depcrate_kv_keyimpl_50 {
() => {
// Module: crate::kv::key
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , T > ToKey for & 'a T where T : ToKey + ? Sized , { fn to_key (& self) -> Key { (* * self) . to_key () } }
};
}
