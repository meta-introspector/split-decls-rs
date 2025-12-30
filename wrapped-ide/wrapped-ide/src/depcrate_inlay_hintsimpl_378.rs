// Generated macro for impl_378 (impl)
macro_rules! Depcrate_inlay_hintsimpl_378 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_378"}
// Dependencies: {}
impl fmt :: Debug for InlayHintLabelPart { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self { text , linked_location : None , tooltip : None | Some (LazyProperty :: Lazy) } => { text . fmt (f) } Self { text , linked_location , tooltip } => f . debug_struct ("InlayHintLabelPart") . field ("text" , text) . field ("linked_location" , linked_location) . field ("tooltip" , & tooltip . as_ref () . map_or ("" , | it | match it { LazyProperty :: Computed (InlayTooltip :: String (it) | InlayTooltip :: Markdown (it) ,) => it , LazyProperty :: Lazy => "" , }) ,) . finish () , } } }
};
}
