// Generated macro for impl_35 (impl)
macro_rules! Depcrate_pkcs8impl_35 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_35"}
// Dependencies: {}
impl fmt :: Debug for PublicKeyBytes { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("PublicKeyBytes(") ? ; for & byte in self . as_ref () { write ! (f , "{byte:02X}") ? ; } f . write_str (")") } }
};
}
