// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl SerializableState for KupynaShortVarCore { type SerializedStateSize = U72 ; # [inline] fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState :: < Self > :: default () ; let (state_dst , len_dst) = serialized_state . split_at_mut (64) ; write_u64_le (& self . state , state_dst) ; len_dst . copy_from_slice (& self . blocks_len . to_le_bytes ()) ; serialized_state } # [inline] fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_state , serialized_block_len) = serialized_state . split :: < U64 > () ; Ok (Self { state : read_u64_le (& serialized_state . 0) , blocks_len : u64 :: from_le_bytes (serialized_block_len . 0) , }) } }
};
}
