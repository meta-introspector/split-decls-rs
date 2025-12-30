// Generated macro for impl_15 (impl)
macro_rules! Depcrate_abi_digesterimpl_15 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_15"}
// Dependencies: {}
impl SerializeTuple for AbiDigester { type Ok = Self ; type Error = DigestError ; fn serialize_element < T : ? Sized + Serialize > (& mut self , data : & T) -> Result < () , DigestError > { self . digest_element (data) } fn end (self) -> DigestResult { Ok (self) } }
};
}
