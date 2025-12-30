// Generated macro for impl_47 (impl)
macro_rules! Depcrate_derimpl_47 {
() => {
// Module: crate::der
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for SignatureRef < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Ok (Self { r : UintRef :: decode (reader) ? , s : UintRef :: decode (reader) ? , }) } }
};
}
