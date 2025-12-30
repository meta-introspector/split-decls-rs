// Generated macro for impl_106 (impl)
macro_rules! Depcrate_kv_valueimpl_106 {
() => {
// Module: crate::kv::value
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'v > Value < 'v > { # [doc = " Try convert this value into an error."] # [cfg (feature = "kv_std")] pub fn to_borrowed_error (& self) -> Option < & (dyn std :: error :: Error + 'static) > { self . inner . to_borrowed_error () } # [doc = " Try convert this value into a borrowed string."] pub fn to_borrowed_str (& self) -> Option < & str > { self . inner . to_borrowed_str () } }
};
}
