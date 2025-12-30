// Generated macro for impl_5 (impl)
macro_rules! Depcrate_heximpl_5 {
() => {
// Module: crate::hex
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: UpperHex for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for component in [& self . R , & self . s] { for byte in component { write ! (f , "{byte:02X}") ? ; } } Ok (()) } }
};
}
