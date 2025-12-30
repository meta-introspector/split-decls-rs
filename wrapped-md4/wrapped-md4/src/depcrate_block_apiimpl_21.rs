// Generated macro for impl_21 (impl)
macro_rules! Depcrate_block_apiimpl_21 {
() => {
// Module: crate::block_api
// Provides: {"impl_21"}
// Dependencies: {}
impl SerializableState for Md4Core { type SerializedStateSize = U24 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState :: < Self > :: default () ; for (val , chunk) in self . state . iter () . zip (serialized_state . chunks_exact_mut (4)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_state [16 ..] . copy_from_slice (& self . block_len . to_le_bytes ()) ; serialized_state } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_state , serialized_block_len) = serialized_state . split :: < U16 > () ; let mut state = [0 ; STATE_LEN] ; for (val , chunk) in state . iter_mut () . zip (serialized_state . chunks_exact (4)) { * val = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let block_len = u64 :: from_le_bytes (* serialized_block_len . as_ref ()) ; Ok (Self { block_len , state }) } }
};
}
