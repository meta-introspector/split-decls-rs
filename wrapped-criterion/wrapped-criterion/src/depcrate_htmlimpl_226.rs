// Generated macro for impl_226 (impl)
macro_rules! Depcrate_htmlimpl_226 {
() => {
// Module: crate::html
// Provides: {"impl_226"}
// Dependencies: {}
impl IndividualBenchmark { fn from_id (output_directory : & Path , path_prefix : & str , id : & BenchmarkId ,) -> IndividualBenchmark { let mut regression_path = PathBuf :: from (output_directory) ; regression_path . push (id . as_directory_name ()) ; regression_path . push ("report") ; regression_path . push ("regression.svg") ; IndividualBenchmark { name : id . as_title () . to_owned () , path : format ! ("{}/{}" , path_prefix , id . as_directory_name ()) , regression_exists : regression_path . is_file () , } } }
};
}
