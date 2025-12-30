// Generated macro for impl_32 (impl)
macro_rules! Depcrate_displayimpl_32 {
() => {
// Module: crate::display
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: UpperHex for HexDisplay < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut hex = [0u8 ; 2] ; for & byte in self . 0 { f . write_str (crate :: upper :: encode_str (& [byte] , & mut hex) ?) ? ; } Ok (()) } }
};
}
