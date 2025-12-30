// Generated macro for impl_117 (impl)
macro_rules! Depcrate_encodeimpl_117 {
() => {
// Module: crate::encode
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > EncodeValue for Box < T > where T : EncodeValue , { fn value_len (& self) -> Result < Length > { T :: value_len (self) } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { T :: encode_value (self , writer) } }
};
}
