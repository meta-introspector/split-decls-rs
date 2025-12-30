// Generated macro for impl_364 (impl)
macro_rules! Depcrate_inlay_hintsimpl_364 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_364"}
// Dependencies: {}
impl fmt :: Display for InlayHintLabel { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . parts . iter () . map (| part | & part . text) . format ("")) } }
};
}
