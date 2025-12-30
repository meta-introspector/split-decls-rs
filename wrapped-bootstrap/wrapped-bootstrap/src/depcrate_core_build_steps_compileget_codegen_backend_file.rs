// Generated macro for get_codegen_backend_file (function)
macro_rules! Depcrate_core_build_steps_compileget_codegen_backend_file {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"get_codegen_backend_file"}
// Dependencies: {}
# [doc = " Gets the path to a dynamic codegen backend library from its build stamp."] pub fn get_codegen_backend_file (stamp : & BuildStamp) -> PathBuf { PathBuf :: from (t ! (fs :: read_to_string (stamp . path ()))) }
};
}
