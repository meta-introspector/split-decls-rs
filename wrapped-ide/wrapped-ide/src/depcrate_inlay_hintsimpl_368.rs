// Generated macro for impl_368 (impl)
macro_rules! Depcrate_inlay_hintsimpl_368 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_368"}
// Dependencies: {}
impl InlayHint { fn closing_paren_after (kind : InlayKind , range : TextRange) -> InlayHint { InlayHint { range , kind , label : InlayHintLabel :: from (")") , text_edit : None , position : InlayHintPosition :: After , pad_left : false , pad_right : false , resolve_parent : None , } } }
};
}
