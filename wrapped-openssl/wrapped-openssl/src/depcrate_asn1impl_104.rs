// Generated macro for impl_104 (impl)
macro_rules! Depcrate_asn1impl_104 {
() => {
// Module: crate::asn1
// Provides: {"impl_104"}
// Dependencies: {}
impl fmt :: Debug for Asn1StringRef { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . as_utf8 () { Ok (openssl_string) => openssl_string . fmt (fmt) , Err (_) => fmt . write_str ("error") , } } }
};
}
