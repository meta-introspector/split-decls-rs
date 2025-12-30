// Generated macro for impl_27 (impl)
macro_rules! Depcrate_pbes1impl_27 {
() => {
// Module: crate::pbes1
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Parameters { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : der :: Header) -> der :: Result < Self > { AnyRef :: decode_value (reader , header) ? . try_into () } }
};
}
