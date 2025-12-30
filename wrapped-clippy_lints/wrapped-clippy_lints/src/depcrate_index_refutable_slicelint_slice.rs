// Generated macro for lint_slice (function)
macro_rules! Depcrate_index_refutable_slicelint_slice {
() => {
// Module: crate::index_refutable_slice
// Provides: {"lint_slice"}
// Dependencies: {}
fn lint_slice (cx : & LateContext < '_ > , slice : & SliceLintInformation) { let used_indices = slice . index_use . iter () . map (| (index , _) | * index) . collect :: < FxIndexSet < _ > > () ; let value_name = | index | format ! ("{}_{}" , slice . ident . name , index) ; if let Some (max_index) = used_indices . iter () . max () { let opt_ref = if slice . needs_ref { "ref " } else { "" } ; let pat_sugg_idents = (0 ..= * max_index) . map (| index | { if used_indices . contains (& index) { format ! ("{opt_ref}{}" , value_name (index)) } else { "_" . to_string () } }) . collect :: < Vec < _ > > () ; let pat_sugg = format ! ("[{}, ..]" , pat_sugg_idents . join (", ")) ; let mut suggestions = Vec :: new () ; if ! slice . pattern_spans . is_empty () { suggestions . extend (slice . pattern_spans . iter () . map (| span | (* span , pat_sugg . clone ()))) ; } if ! slice . index_use . is_empty () { suggestions . extend (slice . index_use . iter () . map (| (index , span) | (* span , value_name (* index)))) ; } span_lint_and_then (cx , INDEX_REFUTABLE_SLICE , slice . ident . span , "this binding can be a slice pattern to avoid indexing" , | diag | { diag . multipart_suggestion ("replace the binding and indexed access with a slice pattern" , suggestions , Applicability :: MaybeIncorrect ,) ; } ,) ; } }
};
}
