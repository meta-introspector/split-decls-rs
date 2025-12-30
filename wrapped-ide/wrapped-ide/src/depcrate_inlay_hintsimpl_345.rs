// Generated macro for impl_345 (impl)
macro_rules! Depcrate_inlay_hintsimpl_345 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_345"}
// Dependencies: {}
impl InlayFieldsToResolve { pub fn from_client_capabilities (client_capability_fields : & FxHashSet < & str >) -> Self { Self { resolve_text_edits : client_capability_fields . contains ("textEdits") , resolve_hint_tooltip : client_capability_fields . contains ("tooltip") , resolve_label_tooltip : client_capability_fields . contains ("label.tooltip") , resolve_label_location : client_capability_fields . contains ("label.location") , resolve_label_command : client_capability_fields . contains ("label.command") , } } pub const fn empty () -> Self { Self { resolve_text_edits : false , resolve_hint_tooltip : false , resolve_label_tooltip : false , resolve_label_location : false , resolve_label_command : false , } } }
};
}
