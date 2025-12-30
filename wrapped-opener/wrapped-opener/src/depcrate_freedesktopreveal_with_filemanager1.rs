// Generated macro for reveal_with_filemanager1 (function)
macro_rules! Depcrate_freedesktopreveal_with_filemanager1 {
() => {
// Module: crate::freedesktop
// Provides: {"reveal_with_filemanager1"}
// Dependencies: {}
fn reveal_with_filemanager1 (path : & Path , connection : & Connection) -> Result < () , OpenError > { let uri = path_to_uri (path) ? ; let proxy = FileManager1Proxy :: new (connection) . map_err (dbus_to_open_error) ? ; proxy . show_items (& [uri] , "") . map_err (dbus_to_open_error) }
};
}
