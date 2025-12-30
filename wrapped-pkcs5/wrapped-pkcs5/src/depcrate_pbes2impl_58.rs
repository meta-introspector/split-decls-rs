// Generated macro for impl_58 (impl)
macro_rules! Depcrate_pbes2impl_58 {
() => {
// Module: crate::pbes2
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Parameters { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : der :: Header) -> der :: Result < Self > { AnyRef :: decode_value (reader , header) ? . try_into () } }
};
}
