// Generated macro for read_file (function)
macro_rules! Depcrate_resource_managerread_file {
() => {
// Module: crate::resource_manager
// Provides: {"read_file"}
// Dependencies: {}
fn read_file (path : & str) -> Result < String , io :: Error > { fs :: read_to_string (path) }
};
}
