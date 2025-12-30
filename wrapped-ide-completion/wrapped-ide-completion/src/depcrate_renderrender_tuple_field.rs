// Generated macro for render_tuple_field (function)
macro_rules! Depcrate_renderrender_tuple_field {
() => {
// Module: crate::render
// Provides: {"render_tuple_field"}
// Dependencies: {}
pub (crate) fn render_tuple_field (ctx : RenderContext < '_ > , receiver : Option < SmolStr > , field : usize , ty : & hir :: Type < '_ > ,) -> CompletionItem { let mut item = CompletionItem :: new (SymbolKind :: Field , ctx . source_range () , field_with_receiver (receiver . as_deref () , & field . to_string ()) , ctx . completion . edition ,) ; item . detail (ty . display (ctx . db () , ctx . completion . display_target) . to_string ()) . lookup_by (field . to_string ()) ; item . set_relevance (CompletionRelevance { is_skipping_completion : receiver . is_some () , .. ctx . completion_relevance () }) ; item . build (ctx . db ()) }
};
}
