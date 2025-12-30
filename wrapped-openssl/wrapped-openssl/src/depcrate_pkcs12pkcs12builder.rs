// Generated macro for Pkcs12Builder (struct)
macro_rules! Depcrate_pkcs12Pkcs12Builder {
() => {
// Module: crate::pkcs12
// Provides: {"Pkcs12Builder"}
// Dependencies: {}
pub struct Pkcs12Builder { name : Option < CString > , pkey : Option < PKey < Private > > , cert : Option < X509 > , ca : Option < Stack < X509 > > , nid_key : Nid , nid_cert : Nid , iter : c_int , mac_iter : c_int , # [cfg (not (boringssl))] mac_md : Option < MessageDigest > , }
};
}
