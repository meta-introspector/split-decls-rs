// Generated macro for impl_33 (impl)
macro_rules! Depcrate_displayimpl_33 {
() => {
// Module: crate::display
// Provides: {"impl_33"}
// Dependencies: {}
impl fmt :: LowerHex for HexDisplay < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut hex = [0u8 ; 2] ; for & byte in self . 0 { f . write_str (crate :: lower :: encode_str (& [byte] , & mut hex) ?) ? ; } Ok (()) } }
};
}
