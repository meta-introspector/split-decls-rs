// Generated macro for impl_16 (impl)
macro_rules! Depcrate_abi_digesterimpl_16 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_16"}
// Dependencies: {}
impl SerializeTupleStruct for AbiDigester { type Ok = Self ; type Error = DigestError ; fn serialize_field < T : ? Sized + Serialize > (& mut self , data : & T) -> Result < () , DigestError > { self . digest_unnamed_field (data) } fn end (self) -> DigestResult { Ok (self) } }
};
}
