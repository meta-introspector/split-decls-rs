// Generated macro for impl_276 (impl)
macro_rules! Depcrate_documentimpl_276 {
() => {
// Module: crate::document
// Provides: {"impl_276"}
// Dependencies: {}
impl Encode for Document { fn encoded_len (& self) -> Result < Length , Error > { Ok (self . len ()) } fn encode (& self , writer : & mut impl Writer) -> Result < () , Error > { writer . write (self . as_bytes ()) } }
};
}
