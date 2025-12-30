// Generated macro for impl_7 (impl)
macro_rules! Depcrate_heximpl_7 {
() => {
// Module: crate::hex
// Provides: {"impl_7"}
// Dependencies: {}
impl fmt :: LowerHex for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for component in [& self . R , & self . s] { for byte in component { write ! (f , "{byte:02x}") ? ; } } Ok (()) } }
};
}
