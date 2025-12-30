// Generated macro for impl_22 (impl)
macro_rules! Depcrate_pbes1impl_22 {
() => {
// Module: crate::pbes1
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Algorithm { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : der :: Header) -> der :: Result < Self > { AlgorithmIdentifierRef :: decode_value (reader , header) ? . try_into () } }
};
}
