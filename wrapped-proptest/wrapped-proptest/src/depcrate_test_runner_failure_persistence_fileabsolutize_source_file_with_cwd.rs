// Generated macro for absolutize_source_file_with_cwd (function)
macro_rules! Depcrate_test_runner_failure_persistence_fileabsolutize_source_file_with_cwd {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"absolutize_source_file_with_cwd"}
// Dependencies: {}
fn absolutize_source_file_with_cwd < 'a > (getcwd : impl FnOnce () -> io :: Result < PathBuf > , source : & 'a Path ,) -> Option < Cow < 'a , Path > > { if source . is_absolute () { Some (Cow :: Borrowed (source)) } else { match getcwd () { Ok (mut cwd) => loop { let joined = cwd . join (source) ; if joined . is_file () { break Some (Cow :: Owned (joined)) ; } if ! cwd . pop () { eprintln ! ("proptest: Failed to find absolute path of \
                         source file '{:?}'. Ensure the test is \
                         being run from somewhere within the crate \
                         directory hierarchy." , source) ; break None ; } } , Err (e) => { eprintln ! ("proptest: Failed to determine current \
                     directory, so the relative source path \
                     '{:?}' cannot be resolved: {}" , source , e) ; None } } } }
};
}
