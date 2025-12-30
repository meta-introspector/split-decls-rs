// Generated macro for optional_std_addr_from_c (function)
macro_rules! Depcrate_ffioptional_std_addr_from_c {
() => {
// Module: crate::ffi
// Provides: {"optional_std_addr_from_c"}
// Dependencies: {}
fn optional_std_addr_from_c (addr : * const sockaddr , addr_len : socklen_t ,) -> Option < SocketAddr > { if addr . is_null () || addr_len == 0 { return None ; } Some ({ let addr = unsafe { slice :: from_raw_parts (addr , addr_len as usize) } ; std_addr_from_c (addr . first () . unwrap () , addr_len) }) }
};
}
