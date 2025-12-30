// Generated macro for impl_74 (impl)
macro_rules! Depcrate_arcimpl_74 {
() => {
// Module: crate::arc
// Provides: {"impl_74"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl From < Arc < str > > for Arc < [u8] > { # [doc = " Converts an atomically reference-counted string slice into a byte slice."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let string: Arc<str> = Arc::from(\"eggplant\");"] # [doc = " let bytes: Arc<[u8]> = Arc::from(string);"] # [doc = " assert_eq!(\"eggplant\".as_bytes(), bytes.as_ref());"] # [doc = " ```"] # [inline] fn from (rc : Arc < str >) -> Self { unsafe { Self :: from_raw (Arc :: into_raw (rc) as * const [u8]) } } }
};
}
