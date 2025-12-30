// Generated macro for impl_626 (impl)
macro_rules! Depcrate_pkcs12impl_626 {
() => {
// Module: crate::pkcs12
// Provides: {"impl_626"}
// Dependencies: {}
impl Pkcs12 { from_der ! { # [doc = " Deserializes a DER-encoded PKCS#12 archive."] # [corresponds (d2i_PKCS12)] from_der , Pkcs12 , ffi :: d2i_PKCS12 } # [doc = " Creates a new builder for a protected pkcs12 certificate."] # [doc = ""] # [doc = " This uses the defaults from the OpenSSL library:"] # [doc = ""] # [doc = " * `nid_key` - `AES_256_CBC` (3.0.0+) or `PBE_WITHSHA1AND3_KEY_TRIPLEDES_CBC`"] # [doc = " * `nid_cert` - `AES_256_CBC` (3.0.0+) or `PBE_WITHSHA1AND40BITRC2_CBC`"] # [doc = " * `iter` - `2048`"] # [doc = " * `mac_iter` - `2048`"] # [doc = " * `mac_md` - `SHA-256` (3.0.0+) or `SHA-1` (`SHA-1` only for BoringSSL)"] pub fn builder () -> Pkcs12Builder { ffi :: init () ; Pkcs12Builder { name : None , pkey : None , cert : None , ca : None , nid_key : Nid :: UNDEF , nid_cert : Nid :: UNDEF , iter : ffi :: PKCS12_DEFAULT_ITER , mac_iter : ffi :: PKCS12_DEFAULT_ITER , # [cfg (not (boringssl))] mac_md : None , } } }
};
}
