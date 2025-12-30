// Generated macro for report_format_mismatch (function)
macro_rules! Depcrate_persist_file_formatreport_format_mismatch {
() => {
// Module: crate::persist::file_format
// Provides: {"report_format_mismatch"}
// Dependencies: {}
fn report_format_mismatch (report_incremental_info : bool , file : & Path , message : & str) { debug ! ("read_file: {}" , message) ; if report_incremental_info { eprintln ! ("[incremental] ignoring cache artifact `{}`: {}" , file . file_name () . unwrap () . to_string_lossy () , message) ; } }
};
}
