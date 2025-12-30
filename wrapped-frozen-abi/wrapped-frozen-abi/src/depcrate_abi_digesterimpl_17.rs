// Generated macro for impl_17 (impl)
macro_rules! Depcrate_abi_digesterimpl_17 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_17"}
// Dependencies: {}
impl SerializeTupleVariant for AbiDigester { type Ok = Self ; type Error = DigestError ; fn serialize_field < T : ? Sized + Serialize > (& mut self , data : & T) -> Result < () , DigestError > { self . digest_unnamed_field (data) } fn end (self) -> DigestResult { Ok (self) } }
};
}
