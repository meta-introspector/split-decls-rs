// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < S > fmt :: Display for HandshakeError < S > where S : Any + fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { try ! (fmt . write_str (self . description ())) ; if let Some (cause) = self . cause () { try ! (write ! (fmt , ": {}" , cause)) ; } Ok (()) } }
};
}
