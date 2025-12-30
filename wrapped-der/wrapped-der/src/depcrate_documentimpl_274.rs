// Generated macro for impl_274 (impl)
macro_rules! Depcrate_documentimpl_274 {
() => {
// Module: crate::document
// Provides: {"impl_274"}
// Dependencies: {}
impl Debug for Document { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Document(") ? ; for byte in self . as_bytes () { write ! (f , "{byte:02X}") ? ; } f . write_str (")") } }
};
}
