// Generated macro for quiche_conn_new_with_tls (function)
macro_rules! Depcrate_ffiquiche_conn_new_with_tls {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_new_with_tls"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_new_with_tls (scid : * const u8 , scid_len : size_t , odcid : * const u8 , odcid_len : size_t , local : & sockaddr , local_len : socklen_t , peer : & sockaddr , peer_len : socklen_t , config : & Config , ssl : * mut c_void , is_server : bool ,) -> * mut Connection { let scid = unsafe { slice :: from_raw_parts (scid , scid_len) } ; let scid = ConnectionId :: from_ref (scid) ; let odcid = if ! odcid . is_null () && odcid_len > 0 { Some (ConnectionId :: from_ref (unsafe { slice :: from_raw_parts (odcid , odcid_len) })) } else { None } ; let local = std_addr_from_c (local , local_len) ; let peer = std_addr_from_c (peer , peer_len) ; let tls = unsafe { tls :: Handshake :: from_ptr (ssl) } ; match Connection :: with_tls (& scid , odcid . as_ref () , local , peer , config , tls , is_server ,) { Ok (c) => Box :: into_raw (Box :: new (c)) , Err (_) => ptr :: null_mut () , } }
};
}
