// Generated macro for impl_18 (impl)
macro_rules! Depcrate_abi_digesterimpl_18 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_18"}
// Dependencies: {}
impl SerializeMap for AbiDigester { type Ok = Self ; type Error = DigestError ; fn serialize_key < T : ? Sized + Serialize > (& mut self , key : & T) -> Result < () , DigestError > { self . update_with_type :: < T > ("key") ; self . create_child () ? . digest_data (key) . map (| _ | ()) } fn serialize_value < T : ? Sized + Serialize > (& mut self , value : & T) -> Result < () , DigestError > { self . update_with_type :: < T > ("value") ; self . create_child () ? . digest_data (value) . map (| _ | ()) } fn end (self) -> DigestResult { Ok (self) } }
};
}
