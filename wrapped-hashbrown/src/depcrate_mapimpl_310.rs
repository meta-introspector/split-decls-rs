// Generated macro for impl_310 (impl)
macro_rules! Depcrate_mapimpl_310 {
() => {
// Module: crate::map
// Provides: {"impl_310"}
// Dependencies: {}
impl < K , V , S , A > Debug for HashMap < K , V , S , A > where K : Debug , V : Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
