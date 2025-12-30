// Generated macro for impl_58 (impl)
macro_rules! Depcrate_bytesimpl_58 {
() => {
// Module: crate::bytes
// Provides: {"impl_58"}
// Dependencies: {}
impl EncodeValue for BytesRef { fn value_len (& self) -> Result < Length > { Ok (self . len ()) } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { writer . write (self . as_ref ()) } }
};
}
