// Generated macro for impl_6 (impl)
macro_rules! Depcrate_heximpl_6 {
() => {
// Module: crate::hex
// Provides: {"impl_6"}
// Dependencies: {}
impl fmt :: Debug for ComponentFormatter < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "0x") ? ; for byte in self . 0 { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}
