// Generated macro for impl_91 (impl)
macro_rules! Depcrate_valueimpl_91 {
() => {
// Module: crate::value
// Provides: {"impl_91"}
// Dependencies: {}
impl From < & HSTRING > for Value { fn from (from : & HSTRING) -> Self { Self { data : Data :: from_slice (as_bytes (from)) , ty : Type :: String , } } }
};
}
