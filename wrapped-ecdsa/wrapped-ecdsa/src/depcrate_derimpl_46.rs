// Generated macro for impl_46 (impl)
macro_rules! Depcrate_derimpl_46 {
() => {
// Module: crate::der
// Provides: {"impl_46"}
// Dependencies: {}
impl EncodeValue for SignatureRef < '_ > { fn value_len (& self) -> der :: Result < Length > { self . r . encoded_len () ? + self . s . encoded_len () ? } fn encode_value (& self , encoder : & mut impl Writer) -> der :: Result < () > { self . r . encode (encoder) ? ; self . s . encode (encoder) ? ; Ok (()) } }
};
}
