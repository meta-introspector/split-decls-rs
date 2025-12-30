// Generated macro for impl_358 (impl)
macro_rules! Depcrate_inlay_hintsimpl_358 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_358"}
// Dependencies: {}
impl InlayHint { fn closing_paren_after (kind : InlayKind , range : TextRange) -> InlayHint { InlayHint { range , kind , label : InlayHintLabel :: from (")") , text_edit : None , position : InlayHintPosition :: After , pad_left : false , pad_right : false , resolve_parent : None , } } }
};
}
