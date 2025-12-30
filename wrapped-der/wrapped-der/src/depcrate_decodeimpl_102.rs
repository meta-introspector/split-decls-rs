// Generated macro for impl_102 (impl)
macro_rules! Depcrate_decodeimpl_102 {
() => {
// Module: crate::decode
// Provides: {"impl_102"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T > DecodeValue < 'a > for Box < T > where T : DecodeValue < 'a > , { type Error = T :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : Header) -> Result < Self , Self :: Error > { Ok (Box :: new (T :: decode_value (reader , header) ?)) } }
};
}
