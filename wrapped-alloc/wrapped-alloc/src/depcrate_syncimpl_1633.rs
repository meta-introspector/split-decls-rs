// Generated macro for impl_1633 (impl)
macro_rules! Depcrate_syncimpl_1633 {
() => {
// Module: crate::sync
// Provides: {"impl_1633"}
// Dependencies: {}
# [stable (feature = "shared_from_str" , since = "1.62.0")] impl From < Arc < str > > for Arc < [u8] > { # [doc = " Converts an atomically reference-counted string slice into a byte slice."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let string: Arc<str> = Arc::from(\"eggplant\");"] # [doc = " let bytes: Arc<[u8]> = Arc::from(string);"] # [doc = " assert_eq!(\"eggplant\".as_bytes(), bytes.as_ref());"] # [doc = " ```"] # [inline] fn from (rc : Arc < str >) -> Self { unsafe { Arc :: from_raw (Arc :: into_raw (rc) as * const [u8]) } } }
};
}
