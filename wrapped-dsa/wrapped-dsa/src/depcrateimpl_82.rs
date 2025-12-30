// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Signature { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : der :: Header) -> der :: Result < Self > { let r = UintRef :: decode (reader) ? ; let s = UintRef :: decode (reader) ? ; let r = BoxedUint :: from_be_slice (r . as_bytes () , r . as_bytes () . len () as u32 * 8) . map_err (| _ | UintRef :: TAG . value_error ()) ? ; let s = BoxedUint :: from_be_slice (s . as_bytes () , s . as_bytes () . len () as u32 * 8) . map_err (| _ | UintRef :: TAG . value_error ()) ? ; Self :: from_components (r , s) . ok_or_else (| | reader . error (UintRef :: TAG . value_error ())) } }
};
}
