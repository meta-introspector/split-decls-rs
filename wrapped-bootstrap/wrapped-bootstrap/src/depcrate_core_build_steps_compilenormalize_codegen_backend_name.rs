// Generated macro for normalize_codegen_backend_name (function)
macro_rules! Depcrate_core_build_steps_compilenormalize_codegen_backend_name {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"normalize_codegen_backend_name"}
// Dependencies: {}
# [doc = " Normalize the name of a dynamic codegen backend library."] pub fn normalize_codegen_backend_name (builder : & Builder < '_ > , path : & Path) -> String { let filename = path . file_name () . unwrap () . to_str () . unwrap () ; let dash = filename . find ('-') . unwrap () ; let dot = filename . find ('.') . unwrap () ; format ! ("{}-{}{}" , & filename [.. dash] , builder . rust_release () , & filename [dot ..]) }
};
}
