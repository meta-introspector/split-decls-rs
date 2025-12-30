// Generated macro for show_blame_entries (function)
macro_rules! Depcrate_repository_blameshow_blame_entries {
() => {
// Module: crate::repository::blame
// Provides: {"show_blame_entries"}
// Dependencies: {}
fn show_blame_entries (mut out : impl std :: io :: Write , outcome : gix :: blame :: Outcome , source_file_name : gix :: bstr :: BString ,) -> Result < () , std :: io :: Error > { for (entry , lines_in_hunk) in outcome . entries_with_lines () { for ((actual_lno , source_lno) , line) in entry . range_in_blamed_file () . zip (entry . range_in_source_file ()) . zip (lines_in_hunk) { write ! (out , "{short_id} {line_no} " , short_id = entry . commit_id . to_hex_with_len (8) , line_no = actual_lno + 1 ,) ? ; let source_file_name = entry . source_file_name . as_ref () . unwrap_or (& source_file_name) ; write ! (out , "{source_file_name} ") ? ; write ! (out , "{src_line_no} {line}" , src_line_no = source_lno + 1) ? ; } } Ok (()) }
};
}
