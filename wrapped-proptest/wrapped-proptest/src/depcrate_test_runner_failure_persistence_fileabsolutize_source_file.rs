// Generated macro for absolutize_source_file (function)
macro_rules! Depcrate_test_runner_failure_persistence_fileabsolutize_source_file {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"absolutize_source_file"}
// Dependencies: {}
# [doc = " Ensure that the source file to use for resolving the location of the persisted"] # [doc = " failing cases file is absolute."] # [doc = ""] # [doc = " The source location can only be used if it is absolute. If `source` is"] # [doc = " not an absolute path, an attempt will be made to determine the absolute"] # [doc = " path based on the current working directory and its parents. If no"] # [doc = " absolute path can be determined, a warning will be printed and proptest"] # [doc = " will continue as if this function had never been called."] # [doc = ""] # [doc = " See [`FileFailurePersistence`](enum.FileFailurePersistence.html) for details on"] # [doc = " how this value is used once it is made absolute."] # [doc = ""] # [doc = " This is normally called automatically by the `proptest!` macro, which"] # [doc = " passes `file!()`."] # [doc = ""] fn absolutize_source_file < 'a > (source : & 'a Path) -> Option < Cow < 'a , Path > > { absolutize_source_file_with_cwd (env :: current_dir , source) }
};
}
