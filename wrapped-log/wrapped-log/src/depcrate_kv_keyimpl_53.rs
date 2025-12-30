// Generated macro for impl_53 (impl)
macro_rules! Depcrate_kv_keyimpl_53 {
() => {
// Module: crate::kv::key
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a , T > ToKey for & 'a T where T : ToKey + ? Sized , { fn to_key (& self) -> Key < '_ > { (* * self) . to_key () } }
};
}
