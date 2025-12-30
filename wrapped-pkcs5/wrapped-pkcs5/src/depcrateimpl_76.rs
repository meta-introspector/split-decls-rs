// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for EncryptionScheme { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (decoder : & mut R , header : Header) -> der :: Result < Self > { AlgorithmIdentifierRef :: decode_value (decoder , header) ? . try_into () } }
};
}
