// Generated macro for impl_23 (impl)
macro_rules! Depcrate_block_apiimpl_23 {
() => {
// Module: crate::block_api
// Provides: {"impl_23"}
// Dependencies: {}
impl < OS : OutputSize > SerializableState for BashHashCore < OS > { type SerializedStateSize = U192 ; fn serialize (& self) -> SerializedState < Self > { let mut res = SerializedState :: < Self > :: default () ; for (src , dst) in self . state . iter () . zip (res . chunks_exact_mut (8)) { dst . copy_from_slice (& src . to_le_bytes ()) ; } res } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let mut state = [0u64 ; STATE_WORDS] ; for (src , dst) in serialized_state . chunks_exact (8) . zip (state . iter_mut ()) { * dst = u64 :: from_le_bytes (src . try_into () . unwrap ()) ; } Ok (Self { state , _pd : PhantomData , }) } }
};
}
