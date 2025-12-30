// Generated macro for is_same_file_system (function)
macro_rules! Depcrate_walkis_same_file_system {
() => {
// Module: crate::walk
// Provides: {"is_same_file_system"}
// Dependencies: {}
# [doc = " Returns true if and only if the given path is on the same device as the"] # [doc = " given root device."] fn is_same_file_system (root_device : u64 , path : & Path) -> Result < bool , Error > { let dent_device = device_num (path) . map_err (| err | Error :: Io (err) . with_path (path)) ? ; Ok (root_device == dent_device) }
};
}
