// Generated macro for impl_128 (impl)
macro_rules! Depcrate_encode_refimpl_128 {
() => {
// Module: crate::encode_ref
// Provides: {"impl_128"}
// Dependencies: {}
impl < T > EncodeValue for EncodeValueRef < '_ , T > where T : EncodeValue , { fn value_len (& self) -> Result < Length > { self . 0 . value_len () } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { self . 0 . encode_value (writer) } }
};
}
