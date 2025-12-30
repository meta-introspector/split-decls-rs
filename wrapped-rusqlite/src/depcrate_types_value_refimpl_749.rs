// Generated macro for impl_749 (impl)
macro_rules! Depcrate_types_value_refimpl_749 {
() => {
// Module: crate::types::value_ref
// Provides: {"impl_749"}
// Dependencies: {}
impl From < ValueRef < '_ > > for Value { # [inline] # [track_caller] fn from (borrowed : ValueRef < '_ >) -> Self { match borrowed { ValueRef :: Null => Self :: Null , ValueRef :: Integer (i) => Self :: Integer (i) , ValueRef :: Real (r) => Self :: Real (r) , ValueRef :: Text (s) => { let s = std :: str :: from_utf8 (s) . expect ("invalid UTF-8") ; Self :: Text (s . to_string ()) } ValueRef :: Blob (b) => Self :: Blob (b . to_vec ()) , } } }
};
}
