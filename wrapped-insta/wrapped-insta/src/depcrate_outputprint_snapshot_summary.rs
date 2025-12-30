// Generated macro for print_snapshot_summary (function)
macro_rules! Depcrate_outputprint_snapshot_summary {
() => {
// Module: crate::output
// Provides: {"print_snapshot_summary"}
// Dependencies: {}
# [doc = " Prints the summary of a snapshot"] pub fn print_snapshot_summary (workspace_root : & Path , snapshot : & Snapshot , snapshot_file : Option < & Path > , line : Option < u32 > ,) { if let Some (snapshot_file) = snapshot_file { let snapshot_file = workspace_root . join (snapshot_file) . strip_prefix (workspace_root) . ok () . map (| x | x . to_path_buf ()) . unwrap_or_else (| | snapshot_file . to_path_buf ()) ; println ! ("Snapshot file: {}" , style (snapshot_file . display ()) . cyan () . underlined ()) ; } if let Some (name) = snapshot . snapshot_name () { println ! ("Snapshot: {}" , style (name) . yellow ()) ; } else { println ! ("Snapshot: {}" , style ("<inline>") . dim ()) ; } if let Some (ref value) = snapshot . metadata () . get_relative_source (workspace_root) { println ! ("Source: {}{}" , style (value . display ()) . cyan () , line . or (snapshot . metadata () . assertion_line ()) . map (| line | format ! (":{}" , style (line) . bold ())) . unwrap_or_default ()) ; } if let Some (ref value) = snapshot . metadata () . input_file () { println ! ("Input file: {}" , style (value) . cyan ()) ; } }
};
}
