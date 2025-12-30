// Generated macro for impl_118 (impl)
macro_rules! Depcrate_encodeimpl_118 {
() => {
// Module: crate::encode
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > EncodeValue for Cow < '_ , T > where T : ToOwned + ? Sized , for < 'a > & 'a T : EncodeValue , { fn value_len (& self) -> Result < Length > { self . as_ref () . value_len () } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { self . as_ref () . encode_value (writer) } }
};
}
