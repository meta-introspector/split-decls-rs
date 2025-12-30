// Generated macro for convert_path_to_socket_address (function)
macro_rules! Depcrateconvert_path_to_socket_address {
() => {
// Module: crate
// Provides: {"convert_path_to_socket_address"}
// Dependencies: {}
# [doc = " Converts a `Path` to its socket address representation."] # [doc = ""] # [doc = " This function is abstract socket-aware."] # [cfg (unix)] # [inline] fn convert_path_to_socket_address (path : & Path) -> io :: Result < rn :: SocketAddrUnix > { # [cfg (any (target_os = "linux" , target_os = "android"))] let address = { use std :: os :: unix :: ffi :: OsStrExt ; let path = path . as_os_str () ; match path . as_bytes () . first () { Some (0) => rn :: SocketAddrUnix :: new_abstract_name (path . as_bytes () . get (1 ..) . unwrap ()) ? , _ => rn :: SocketAddrUnix :: new (path) ? , } } ; # [cfg (not (any (target_os = "linux" , target_os = "android")))] let address = rn :: SocketAddrUnix :: new (path) ? ; Ok (address) }
};
}
