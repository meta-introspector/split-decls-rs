// Generated macro for impl_327 (impl)
macro_rules! Depcrate_mapimpl_327 {
() => {
// Module: crate::map
// Provides: {"impl_327"}
// Dependencies: {}
impl < K : Debug , V : Debug , A : Allocator > fmt :: Debug for IntoKeys < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (k , _) | k)) . finish () } }
};
}
