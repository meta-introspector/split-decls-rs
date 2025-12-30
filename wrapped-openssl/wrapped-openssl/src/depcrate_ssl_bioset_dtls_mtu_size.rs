// Generated macro for set_dtls_mtu_size (function)
macro_rules! Depcrate_ssl_bioset_dtls_mtu_size {
() => {
// Module: crate::ssl::bio
// Provides: {"set_dtls_mtu_size"}
// Dependencies: {}
pub unsafe fn set_dtls_mtu_size < S > (bio : * mut BIO , mtu_size : usize) { if mtu_size as u64 > c_long :: MAX as u64 { panic ! ("Given MTU size {} can't be represented in a positive `c_long` range" , mtu_size) } state :: < S > (bio) . dtls_mtu_size = mtu_size as c_long ; }
};
}
