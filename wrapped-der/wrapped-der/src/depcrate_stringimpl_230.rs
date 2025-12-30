// Generated macro for impl_230 (impl)
macro_rules! Depcrate_stringimpl_230 {
() => {
// Module: crate::string
// Provides: {"impl_230"}
// Dependencies: {}
impl EncodeValue for StringRef { fn value_len (& self) -> Result < Length > { Ok (self . len ()) } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { writer . write (self . as_ref ()) } }
};
}
