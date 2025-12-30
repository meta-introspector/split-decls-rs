// Generated macro for impl_21 (impl)
macro_rules! Depcrate_block_apiimpl_21 {
() => {
// Module: crate::block_api
// Provides: {"impl_21"}
// Dependencies: {}
impl SerializableState for BeltHashCore { type SerializedStateSize = U64 ; fn serialize (& self) -> SerializedState < Self > { let mut dst = SerializedState :: < Self > :: default () ; let (r_dst , tail) = dst . split_at_mut (16) ; let (s_dst , h_dst) = tail . split_at_mut (16) ; r_dst . copy_from_slice (& self . r . to_le_bytes ()) ; write_u32s (& self . s , s_dst) ; write_u32s (& self . h , h_dst) ; dst } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (r_src , tail) = serialized_state . split_at (16) ; let (s_src , h_src) = tail . split_at (16) ; Ok (Self { r : u128 :: from_le_bytes (r_src . try_into () . unwrap ()) , s : read_u32s (s_src) , h : read_u32s (h_src) , }) } }
};
}
