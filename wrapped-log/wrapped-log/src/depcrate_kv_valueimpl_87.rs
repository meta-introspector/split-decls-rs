// Generated macro for impl_87 (impl)
macro_rules! Depcrate_kv_valueimpl_87 {
() => {
// Module: crate::kv::value
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a , T > ToValue for & 'a T where T : ToValue + ? Sized , { fn to_value (& self) -> Value { (* * self) . to_value () } }
};
}
