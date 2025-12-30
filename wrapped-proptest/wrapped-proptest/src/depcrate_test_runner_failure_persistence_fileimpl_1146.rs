// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_test_runner_failure_persistence_fileimpl_1146 {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"impl_1146"}
// Dependencies: {}
impl FileFailurePersistence { # [doc = " Given the nominal source path, determine the location of the failure"] # [doc = " persistence file, if any."] pub (super) fn resolve (& self , source : Option < & Path >) -> Option < PathBuf > { let source = source . and_then (absolutize_source_file) ; match * self { Off => None , SourceParallel (sibling) => match source { Some (source_path) => { let mut dir = Cow :: into_owned (source_path . clone ()) ; let mut found = false ; while dir . pop () { if dir . join ("lib.rs") . is_file () || dir . join ("main.rs") . is_file () { found = true ; break ; } } if ! found { eprintln ! ("proptest: FileFailurePersistence::SourceParallel set, \
                             but failed to find lib.rs or main.rs") ; WithSource (sibling) . resolve (Some (& * source_path)) } else { let suffix = source_path . strip_prefix (& dir) . expect ("parent of source is not a prefix of it?") . to_owned () ; let mut result = dir ; let _ = result . pop () ; result . push (sibling) ; result . push (& suffix) ; result . set_extension ("txt") ; Some (result) } } None => { eprintln ! ("proptest: FileFailurePersistence::SourceParallel set, \
                         but no source file known") ; None } } , WithSource (extension) => match source { Some (source_path) => { let mut result = Cow :: into_owned (source_path) ; result . set_extension (extension) ; Some (result) } None => { eprintln ! ("proptest: FileFailurePersistence::WithSource set, \
                         but no source file known") ; None } } , Direct (path) => Some (Path :: new (path) . to_owned ()) , _NonExhaustive => { panic ! ("FailurePersistence set to _NonExhaustive") } } } }
};
}
