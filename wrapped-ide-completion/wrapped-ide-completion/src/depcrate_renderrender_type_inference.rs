// Generated macro for render_type_inference (function)
macro_rules! Depcrate_renderrender_type_inference {
() => {
// Module: crate::render
// Provides: {"render_type_inference"}
// Dependencies: {}
pub (crate) fn render_type_inference (ty_string : String , ctx : & CompletionContext < '_ > ,) -> CompletionItem { let mut builder = CompletionItem :: new (CompletionItemKind :: InferredType , ctx . source_range () , ty_string , ctx . edition ,) ; builder . set_relevance (CompletionRelevance { type_match : Some (CompletionRelevanceTypeMatch :: Exact) , exact_name_match : true , .. Default :: default () }) ; builder . build (ctx . db) }
};
}
