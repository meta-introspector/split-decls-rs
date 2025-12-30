// Generated macro for run_predicate_script (function)
macro_rules! Depcraterun_predicate_script {
() => {
// Module: crate
// Provides: {"run_predicate_script"}
// Dependencies: {}
fn run_predicate_script (header : & fuzzers :: HeaderC ,) -> Result < Output , Box < dyn Error > > { let dir = Builder :: new () . prefix ("bindgen_prop") . tempdir () ? ; let header_path = dir . path () . join ("prop_test.h") ; let mut header_file = File :: create (& header_path) ? ; header_file . write_all (header . to_string () . as_bytes ()) ? ; header_file . sync_all () ? ; let header_path_string = header_path . into_os_string () . into_string () . map_err (| _ | "error converting path into String") ? ; let mut predicate_script_path = PathBuf :: from (env ! ("CARGO_MANIFEST_DIR")) ; predicate_script_path . push ("../../csmith-fuzzing/predicate.py") ; let predicate_script_path_string = predicate_script_path . into_os_string () . into_string () . map_err (| _ | "error converting path into String") ? ; if let Some (ref path) = CONTEXT . lock () . unwrap () . output_path { Command :: new ("cp") . arg ("-a") . arg (dir . path () . to_str () . unwrap ()) . arg (path) . output () ? ; } Ok (Command :: new (predicate_script_path_string) . arg (& header_path_string) . output () ?) }
};
}
