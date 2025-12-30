// Generated macro for copy_and_stamp (function)
macro_rules! Depcrate_core_build_steps_compilecopy_and_stamp {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"copy_and_stamp"}
// Dependencies: {}
fn copy_and_stamp (builder : & Builder < '_ > , libdir : & Path , sourcedir : & Path , name : & str , target_deps : & mut Vec < (PathBuf , DependencyType) > , dependency_type : DependencyType ,) { let target = libdir . join (name) ; builder . copy_link (& sourcedir . join (name) , & target , FileType :: Regular) ; target_deps . push ((target , dependency_type)) ; }
};
}
