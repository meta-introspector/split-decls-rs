// Generated macro for impl_8 (impl)
macro_rules! Depcrate_heximpl_8 {
() => {
// Module: crate::hex
// Provides: {"impl_8"}
// Dependencies: {}
impl fmt :: UpperHex for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for component in [& self . R , & self . s] { for byte in component { write ! (f , "{byte:02X}") ? ; } } Ok (()) } }
};
}
