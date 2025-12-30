// Generated macro for impl_1503 (impl)
macro_rules! Depcrate_registry_cache_controlimpl_1503 {
() => {
// Module: crate::registry::cache_control
// Provides: {"impl_1503"}
// Dependencies: {}
impl CacheControl { # [doc = " Get 'Cache-Control' header value."] # [must_use] pub fn value (& self) -> Option < String > { let mut value = if self . max_age > 0 { format ! ("max-age={}" , self . max_age) } else if self . max_age == - 1 { "no-cache" . to_string () } else { String :: new () } ; if ! self . public { if ! value . is_empty () { value += ", " ; } value += "private" ; } if ! value . is_empty () { Some (value) } else { None } } }
};
}
