// Generated macro for impl_13 (impl)
macro_rules! Depcrate_attrimpl_13 {
() => {
// Module: crate::attr
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for MessageDigest { type Error = < OctetString as DecodeValue < 'a > > :: Error ; # [inline] fn decode_value < R : der :: Reader < 'a > > (reader : & mut R , header : der :: Header) -> der :: Result < Self > { OctetString :: decode_value (reader , header) . map (Self) } }
};
}
