// Generated macro for impl_19 (impl)
macro_rules! Depcrate_abi_digesterimpl_19 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_19"}
// Dependencies: {}
impl SerializeStruct for AbiDigester { type Ok = Self ; type Error = DigestError ; fn serialize_field < T : ? Sized + Serialize > (& mut self , key : Sstr , data : & T ,) -> Result < () , DigestError > { self . digest_named_field (key , data) } fn end (self) -> DigestResult { Ok (self) } }
};
}
