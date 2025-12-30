// Generated macro for impl_66 (impl)
macro_rules! Depcrate_renderer_renderimpl_66 {
() => {
// Module: crate::renderer::render
// Provides: {"impl_66"}
// Dependencies: {}
impl DisplaySuggestion { fn new (complete : & str , patches : & [TrimmedPatch < '_ >] , sm : & SourceMap < '_ >) -> Self { let has_deletion = patches . iter () . any (| p | p . is_deletion (sm) || p . is_destructive_replacement (sm)) ; let is_multiline = complete . lines () . count () > 1 ; if has_deletion && ! is_multiline { DisplaySuggestion :: Diff } else if patches . len () == 1 && patches . first () . map_or (false , | p | { p . replacement . ends_with ('\n') && p . replacement . trim () == complete . trim () }) { DisplaySuggestion :: Add } else if (patches . len () != 1 || patches [0] . replacement . trim () != complete . trim ()) && ! is_multiline { DisplaySuggestion :: Underline } else { DisplaySuggestion :: None } } }
};
}
