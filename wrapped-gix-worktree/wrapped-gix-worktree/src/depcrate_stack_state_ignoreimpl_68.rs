// Generated macro for impl_68 (impl)
macro_rules! Depcrate_stack_state_ignoreimpl_68 {
() => {
// Module: crate::stack::state::ignore
// Provides: {"impl_68"}
// Dependencies: {}
impl Ignore { # [doc = " Configure gitignore file matching by providing the immutable groups being `overrides` and `globals`, while letting the directory"] # [doc = " stack be dynamic."] # [doc = ""] # [doc = " The `exclude_file_name_for_directories` is an optional override for the filename to use when checking per-directory"] # [doc = " ignore files within the repository, defaults to`.gitignore`."] # [doc = ""] # [doc = " `parse` controls how to parse ignore files."] pub fn new (overrides : IgnoreMatchGroup , globals : IgnoreMatchGroup , exclude_file_name_for_directories : Option < & BStr > , source : Source , parse : gix_ignore :: search :: Ignore ,) -> Self { Ignore { overrides , globals , stack : Default :: default () , matched_directory_patterns_stack : Vec :: with_capacity (6) , exclude_file_name_for_directories : exclude_file_name_for_directories . map_or_else (| | ".gitignore" . into () , ToOwned :: to_owned) , source , parse , } } }
};
}
