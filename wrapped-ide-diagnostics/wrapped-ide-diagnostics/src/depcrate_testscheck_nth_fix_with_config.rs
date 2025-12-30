// Generated macro for check_nth_fix_with_config (function)
macro_rules! Depcrate_testscheck_nth_fix_with_config {
() => {
// Module: crate::tests
// Provides: {"check_nth_fix_with_config"}
// Dependencies: {}
# [track_caller] fn check_nth_fix_with_config (config : DiagnosticsConfig , nth : usize , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let after = trim_indent (ra_fixture_after) ; let (db , file_position) = RootDatabase :: with_position (ra_fixture_before) ; let diagnostic = hir :: attach_db (& db , | | { super :: full_diagnostics (& db , & config , & AssistResolveStrategy :: All , file_position . file_id . file_id (& db) ,) . pop () . expect ("no diagnostics") }) ; let fix = & diagnostic . fixes . unwrap_or_else (| | panic ! ("{:?} diagnostic misses fixes" , diagnostic . code)) [nth] ; let actual = { let source_change = fix . source_change . as_ref () . unwrap () ; let file_id = * source_change . source_file_edits . keys () . next () . unwrap () ; let mut actual = db . file_text (file_id) . text (& db) . to_string () ; for (edit , snippet_edit) in source_change . source_file_edits . values () { edit . apply (& mut actual) ; if let Some (snippet_edit) = snippet_edit { snippet_edit . apply (& mut actual) ; } } actual } ; assert ! (fix . target . contains_inclusive (file_position . offset) , "diagnostic fix range {:?} does not touch cursor position {:?}" , fix . target , file_position . offset) ; assert_eq_text ! (& after , & actual) ; }
};
}
