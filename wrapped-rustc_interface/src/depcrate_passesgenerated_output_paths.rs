// Generated macro for generated_output_paths (function)
macro_rules! Depcrate_passesgenerated_output_paths {
() => {
// Module: crate::passes
// Provides: {"generated_output_paths"}
// Dependencies: {}
fn generated_output_paths (tcx : TyCtxt < '_ > , outputs : & OutputFilenames , exact_name : bool , crate_name : Symbol ,) -> Vec < PathBuf > { let sess = tcx . sess ; let mut out_filenames = Vec :: new () ; for output_type in sess . opts . output_types . keys () { let out_filename = outputs . path (* output_type) ; let file = out_filename . as_path () . to_path_buf () ; match * output_type { OutputType :: Exe if ! exact_name => { for crate_type in tcx . crate_types () . iter () { let p = filename_for_input (sess , * crate_type , crate_name , outputs) ; out_filenames . push (p . as_path () . to_path_buf ()) ; } } OutputType :: DepInfo if sess . opts . unstable_opts . dep_info_omit_d_target => { } OutputType :: DepInfo if out_filename . is_stdout () => { } _ => { out_filenames . push (file) ; } } } out_filenames }
};
}
