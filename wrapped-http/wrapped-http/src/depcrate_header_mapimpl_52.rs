// Generated macro for impl_52 (impl)
macro_rules! Depcrate_header_mapimpl_52 {
() => {
// Module: crate::header::map
// Provides: {"impl_52"}
// Dependencies: {}
impl HeaderMap { # [doc = " Create an empty `HeaderMap`."] # [doc = ""] # [doc = " The map will be created without any capacity. This function will not"] # [doc = " allocate."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::HeaderMap;"] # [doc = " let map = HeaderMap::new();"] # [doc = ""] # [doc = " assert!(map.is_empty());"] # [doc = " assert_eq!(0, map.capacity());"] # [doc = " ```"] # [inline] pub fn new () -> Self { Self :: default () } }
};
}
