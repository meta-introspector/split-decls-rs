// Generated macro for impl_173 (impl)
macro_rules! Depcrate_encodingimpl_173 {
() => {
// Module: crate::encoding
// Provides: {"impl_173"}
// Dependencies: {}
impl std :: error :: Error for EncodingError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Self :: Utf8 (e) => Some (e) , # [cfg (feature = "encoding")] Self :: Other (_) => None , } } }
};
}
