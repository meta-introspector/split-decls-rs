// Generated macro for impl_21 (impl)
macro_rules! Depcrate_block_apiimpl_21 {
() => {
// Module: crate::block_api
// Provides: {"impl_21"}
// Dependencies: {}
impl SerializableState for Md2Core { type SerializedStateSize = U64 ; fn serialize (& self) -> SerializedState < Self > { let checksum : Block < Self > = self . checksum . into () ; Array :: < _ , U48 > :: from (self . x) . concat (checksum) } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_x , serialized_checksum) = serialized_state . split :: < U48 > () ; Ok (Self { x : * serialized_x . as_ref () , checksum : serialized_checksum . 0 , }) } }
};
}
