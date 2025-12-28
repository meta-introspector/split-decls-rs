macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! check_has_fix {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_has_fix (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let after = trim_indent (ra_fixture_after) ; let (db , file_position) = RootDatabase :: with_position (ra_fixture_before) ; let mut conf = DiagnosticsConfig :: test_sample () ; conf . expr_fill_default = ExprFillDefaultMode :: Default ; let fix = hir :: attach_db (& db , | | { super :: full_diagnostics (& db , & conf , & AssistResolveStrategy :: All , file_position . file_id . file_id (& db) ,) }) . into_iter () . find (| d | { d . fixes . as_ref () . and_then (| fixes | { fixes . iter () . find (| fix | { if ! fix . target . contains_inclusive (file_position . offset) { return false ; } let actual = { let source_change = fix . source_change . as_ref () . unwrap () ; let file_id = * source_change . source_file_edits . keys () . next () . unwrap () ; let mut actual = db . file_text (file_id) . text (& db) . to_string () ; for (edit , snippet_edit) in source_change . source_file_edits . values () { edit . apply (& mut actual) ; if let Some (snippet_edit) = snippet_edit { snippet_edit . apply (& mut actual) ; } } actual } ; after == actual }) }) . is_some () }) ; assert ! (fix . is_some () , "no diagnostic with desired fix") ; }
    };
}

check_has_fix!()