// Generated macro for impl_88 (impl)
macro_rules! Depcrate_valueimpl_88 {
() => {
// Module: crate::value
// Provides: {"impl_88"}
// Dependencies: {}
impl From < & str > for Value { fn from (from : & str) -> Self { Self { data : Data :: from_slice (pcwstr (from) . as_bytes ()) , ty : Type :: String , } } }
};
}
