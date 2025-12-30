// Generated macro for select_next_proto (function)
macro_rules! Depcrate_sslselect_next_proto {
() => {
// Module: crate::ssl
// Provides: {"select_next_proto"}
// Dependencies: {}
# [doc = " A standard implementation of protocol selection for Application Layer Protocol Negotiation"] # [doc = " (ALPN)."] # [doc = ""] # [doc = " `server` should contain the server's list of supported protocols and `client` the client's. They"] # [doc = " must both be in the ALPN wire format. See the documentation for"] # [doc = " [`SslContextBuilder::set_alpn_protos`] for details."] # [doc = ""] # [doc = " It will select the first protocol supported by the server which is also supported by the client."] # [doc = ""] # [doc = " [`SslContextBuilder::set_alpn_protos`]: struct.SslContextBuilder.html#method.set_alpn_protos"] # [corresponds (SSL_select_next_proto)] pub fn select_next_proto < 'a > (server : & 'a [u8] , client : & 'a [u8]) -> Option < & 'a [u8] > { unsafe { let mut out = ptr :: null_mut () ; let mut outlen = 0 ; let r = ffi :: SSL_select_next_proto (& mut out , & mut outlen , server . as_ptr () , server . len () as c_uint , client . as_ptr () , client . len () as c_uint ,) ; if r == ffi :: OPENSSL_NPN_NEGOTIATED { Some (util :: from_raw_parts (out as * const u8 , outlen as usize)) } else { None } } }
};
}
