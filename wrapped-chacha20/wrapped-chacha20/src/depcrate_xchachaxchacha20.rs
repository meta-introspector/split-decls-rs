// Generated macro for XChaCha20 (type)
macro_rules! Depcrate_xchachaXChaCha20 {
() => {
// Module: crate::xchacha
// Provides: {"XChaCha20"}
// Dependencies: {}
# [doc = " XChaCha is a ChaCha20 variant with an extended 192-bit (24-byte) nonce."] # [doc = ""] # [doc = " The construction is an adaptation of the same techniques used by"] # [doc = " XChaCha as described in the paper \"Extending the Salsa20 Nonce\","] # [doc = " applied to the 96-bit nonce variant of ChaCha20, and derive a"] # [doc = " separate subkey/nonce for each extended nonce:"] # [doc = ""] # [doc = " <https://cr.yp.to/snuffle/xsalsa-20081128.pdf>"] # [doc = ""] # [doc = " No authoritative specification exists for XChaCha20, however the"] # [doc = " construction has \"rough consensus and running code\" in the form of"] # [doc = " several interoperable libraries and protocols (e.g. libsodium, WireGuard)"] # [doc = " and is documented in an (expired) IETF draft:"] # [doc = ""] # [doc = " <https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-xchacha>"] pub type XChaCha20 = StreamCipherCoreWrapper < XChaChaCore < R20 > > ;
};
}
