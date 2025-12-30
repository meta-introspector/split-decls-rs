// Generated macro for impl_902 (impl)
macro_rules! Depcrate_util_any_valueimpl_902 {
() => {
// Module: crate::util::any_value
// Provides: {"impl_902"}
// Dependencies: {}
impl std :: fmt :: Debug for AnyValueId { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { # [cfg (not (debug_assertions))] { self . type_id . fmt (f) } # [cfg (debug_assertions)] { f . debug_struct (self . type_name) . finish () } } }
};
}
