// Generated macro for check_edit_with_config (function)
macro_rules! Depcrate_testscheck_edit_with_config {
() => {
// Module: crate::tests
// Provides: {"check_edit_with_config"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_edit_with_config (config : CompletionConfig < '_ > , what : & str , ra_fixture_before : & str , ra_fixture_after : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; let (db , position) = position (ra_fixture_before) ; let completions : Vec < CompletionItem > = hir :: attach_db (& db , | | crate :: completions (& db , & config , position , None) . unwrap ()) ; let Some ((completion ,)) = completions . iter () . filter (| it | it . lookup () == what) . collect_tuple () else { panic ! ("can't find {what:?} completion in {completions:#?}") } ; let mut actual = db . file_text (position . file_id) . text (& db) . to_string () ; let mut combined_edit = completion . text_edit . clone () ; resolve_completion_edits (& db , & config , position , completion . import_to_add . iter () . cloned ()) . into_iter () . flatten () . for_each (| text_edit | { combined_edit . union (text_edit) . expect ("Failed to apply completion resolve changes: change ranges overlap, but should not" ,) }) ; combined_edit . apply (& mut actual) ; assert_eq_text ! (& ra_fixture_after , & actual) }
};
}
