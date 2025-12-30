// Generated macro for impl_333 (impl)
macro_rules! Depcrate_mapimpl_333 {
() => {
// Module: crate::map
// Provides: {"impl_333"}
// Dependencies: {}
impl < K , V : Debug , A : Allocator > fmt :: Debug for IntoValues < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , v) | v)) . finish () } }
};
}
