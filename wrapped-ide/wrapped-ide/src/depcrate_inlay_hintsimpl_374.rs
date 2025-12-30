// Generated macro for impl_374 (impl)
macro_rules! Depcrate_inlay_hintsimpl_374 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_374"}
// Dependencies: {}
impl fmt :: Display for InlayHintLabel { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . parts . iter () . map (| part | & part . text) . format ("")) } }
};
}
