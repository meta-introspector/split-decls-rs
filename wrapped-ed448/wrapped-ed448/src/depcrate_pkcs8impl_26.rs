// Generated macro for impl_26 (impl)
macro_rules! Depcrate_pkcs8impl_26 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Debug for PublicKeyBytes { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("PublicKeyBytes(") ? ; for & byte in self . as_ref () { write ! (f , "{byte:02X}") ? ; } f . write_str (")") } }
};
}
