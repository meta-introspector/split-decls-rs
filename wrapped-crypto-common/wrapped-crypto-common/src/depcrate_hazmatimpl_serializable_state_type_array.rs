// Generated macro for impl_serializable_state_type_array (macro)
macro_rules! Depcrate_hazmatimpl_serializable_state_type_array {
() => {
// Module: crate::hazmat
// Provides: {"impl_serializable_state_type_array"}
// Dependencies: {}
macro_rules ! impl_serializable_state_type_array { ($ type : ty , $ type_size : ty , $ n : ty) => { impl SerializableState for [$ type ; <$ n >:: USIZE] { type SerializedStateSize = Prod <$ n , $ type_size >; fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState ::< Self >:: default () ; for (val , chunk) in self . iter () . zip (serialized_state . chunks_exact_mut (<$ type_size >:: USIZE)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_state } fn deserialize (serialized_state : & SerializedState < Self >,) -> Result < Self , DeserializeStateError > { let mut array = [0 ; <$ n >:: USIZE] ; for (val , chunk) in array . iter_mut () . zip (serialized_state . chunks_exact (<$ type_size >:: USIZE)) { * val = <$ type >:: from_le_bytes (chunk . try_into () . unwrap ()) ; } Ok (array) } } } ; }
};
}
