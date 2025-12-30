// Generated macro for reveal_with_dbus (function)
macro_rules! Depcrate_freedesktopreveal_with_dbus {
() => {
// Module: crate::freedesktop
// Provides: {"reveal_with_dbus"}
// Dependencies: {}
pub (crate) fn reveal_with_dbus (path : & Path) -> Result < () , OpenError > { let connection = Connection :: session () . map_err (dbus_to_open_error) ? ; reveal_with_filemanager1 (path , & connection) . or_else (| _ | reveal_with_open_uri_portal (path , & connection)) }
};
}
