// Generated macro for impl_90 (impl)
macro_rules! Depcrate_kv_valueimpl_90 {
() => {
// Module: crate::kv::value
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a , T > ToValue for & 'a T where T : ToValue + ? Sized , { fn to_value (& self) -> Value < '_ > { (* * self) . to_value () } }
};
}
