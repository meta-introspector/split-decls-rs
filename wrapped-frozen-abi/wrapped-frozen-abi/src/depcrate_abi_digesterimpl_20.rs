// Generated macro for impl_20 (impl)
macro_rules! Depcrate_abi_digesterimpl_20 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_20"}
// Dependencies: {}
impl SerializeStructVariant for AbiDigester { type Ok = Self ; type Error = DigestError ; fn serialize_field < T : ? Sized + Serialize > (& mut self , key : Sstr , data : & T ,) -> Result < () , DigestError > { self . digest_named_field (key , data) } fn end (self) -> DigestResult { Ok (self) } }
};
}
