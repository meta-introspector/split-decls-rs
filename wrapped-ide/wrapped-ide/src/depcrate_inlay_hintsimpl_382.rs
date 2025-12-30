// Generated macro for impl_382 (impl)
macro_rules! Depcrate_inlay_hintsimpl_382 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_382"}
// Dependencies: {}
impl InlayHintLabelBuilder < '_ > { fn make_new_part (& mut self) { let text = take (& mut self . last_part) ; if ! text . is_empty () { self . result . parts . push (InlayHintLabelPart { text , linked_location : self . location . take () , tooltip : None , }) ; } } fn finish (mut self) -> InlayHintLabel { self . make_new_part () ; self . result } }
};
}
