// Generated macro for render_resolution_simple_ (function)
macro_rules! Depcrate_renderrender_resolution_simple_ {
() => {
// Module: crate::render
// Provides: {"render_resolution_simple_"}
// Dependencies: {}
fn render_resolution_simple_ (ctx : RenderContext < '_ > , local_name : & hir :: Name , import_to_add : Option < LocatedImport > , resolution : ScopeDef ,) -> Builder { let _p = tracing :: info_span ! ("render_resolution_simple_") . entered () ; let db = ctx . db () ; let ctx = ctx . import_to_add (import_to_add) ; let kind = res_to_kind (resolution) ; let mut item = CompletionItem :: new (kind , ctx . source_range () , local_name . as_str () . to_smolstr () , ctx . completion . edition ,) ; item . set_relevance (ctx . completion_relevance ()) . set_documentation (scope_def_docs (db , resolution)) . set_deprecated (scope_def_is_deprecated (& ctx , resolution)) ; if let Some (import_to_add) = ctx . import_to_add { item . add_import (import_to_add) ; } item . doc_aliases (ctx . doc_aliases) ; item }
};
}
