// Generated macro for apple_darwin_sign_file (function)
macro_rules! Depcrate_core_build_steps_compileapple_darwin_sign_file {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"apple_darwin_sign_file"}
// Dependencies: {}
fn apple_darwin_sign_file (builder : & Builder < '_ > , file_path : & Path) { command ("codesign") . arg ("-f") . arg ("-s") . arg ("-") . arg (file_path) . run (builder) ; }
};
}
