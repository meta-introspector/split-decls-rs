// Generated macro for impl_102 (impl)
macro_rules! Depcrate_kv_valueimpl_102 {
() => {
// Module: crate::kv::value
// Provides: {"impl_102"}
// Dependencies: {}
impl < T > ToValue for Option < T > where T : ToValue , { fn to_value (& self) -> Value < '_ > { match * self { Some (ref value) => value . to_value () , None => Value :: from_inner (()) , } } }
};
}
