// Generated macro for quiche_connect (function)
macro_rules! Depcrate_ffiquiche_connect {
() => {
// Module: crate::ffi
// Provides: {"quiche_connect"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_connect (server_name : * const c_char , scid : * const u8 , scid_len : size_t , local : & sockaddr , local_len : socklen_t , peer : & sockaddr , peer_len : socklen_t , config : & mut Config ,) -> * mut Connection { let server_name = if server_name . is_null () { None } else { Some (unsafe { ffi :: CStr :: from_ptr (server_name) . to_str () . unwrap () }) } ; let scid = unsafe { slice :: from_raw_parts (scid , scid_len) } ; let scid = ConnectionId :: from_ref (scid) ; let local = std_addr_from_c (local , local_len) ; let peer = std_addr_from_c (peer , peer_len) ; match connect (server_name , & scid , local , peer , config) { Ok (c) => Box :: into_raw (Box :: new (c)) , Err (_) => ptr :: null_mut () , } }
};
}
