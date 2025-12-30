// Generated macro for impl_493 (impl)
macro_rules! Depcrate_readerimpl_493 {
() => {
// Module: crate::reader
// Provides: {"impl_493"}
// Dependencies: {}
# [cfg (feature = "encoding")] impl EncodingRef { # [inline] const fn encoding (& self) -> & 'static Encoding { match self { Self :: Implicit (e) => e , Self :: Explicit (e) => e , Self :: BomDetected (e) => e , Self :: XmlDetected (e) => e , } } # [inline] const fn can_be_refined (& self) -> bool { match self { Self :: Implicit (_) | Self :: BomDetected (_) => true , Self :: Explicit (_) | Self :: XmlDetected (_) => false , } } }
};
}
