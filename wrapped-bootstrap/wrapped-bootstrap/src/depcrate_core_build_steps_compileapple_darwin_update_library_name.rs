// Generated macro for apple_darwin_update_library_name (function)
macro_rules! Depcrate_core_build_steps_compileapple_darwin_update_library_name {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"apple_darwin_update_library_name"}
// Dependencies: {}
fn apple_darwin_update_library_name (builder : & Builder < '_ > , library_path : & Path , new_name : & str) { command ("install_name_tool") . arg ("-id") . arg (new_name) . arg (library_path) . run (builder) ; }
};
}
