// Generated macro for impl_301 (impl)
macro_rules! Depcrate_source_changeimpl_301 {
() => {
// Module: crate::source_change
// Provides: {"impl_301"}
// Dependencies: {}
impl PlaceSnippet { fn finalize_position (self) -> Vec < Snippet > { match self { PlaceSnippet :: Before (it) => vec ! [Snippet :: Tabstop (it . text_range () . start ())] , PlaceSnippet :: After (it) => vec ! [Snippet :: Tabstop (it . text_range () . end ())] , PlaceSnippet :: Over (it) => vec ! [Snippet :: Placeholder (it . text_range ())] , PlaceSnippet :: OverGroup (it) => { vec ! [Snippet :: PlaceholderGroup (it . into_iter () . map (| it | it . text_range ()) . collect ())] } } } }
};
}
