// Generated macro for impl_95 (impl)
macro_rules! Depcrate_decodeimpl_95 {
() => {
// Module: crate::decode
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , T > Decode < 'a > for T where T : DecodeValue < 'a > + FixedTag + 'a , { type Error = < T as DecodeValue < 'a > > :: Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> Result < T , < T as DecodeValue < 'a > > :: Error > { let header = Header :: decode (reader) ? ; header . tag () . assert_eq (T :: TAG) ? ; read_value (reader , header , T :: decode_value) } }
};
}
