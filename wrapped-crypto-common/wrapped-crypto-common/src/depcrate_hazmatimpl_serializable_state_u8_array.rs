// Generated macro for impl_serializable_state_u8_array (macro)
macro_rules! Depcrate_hazmatimpl_serializable_state_u8_array {
() => {
// Module: crate::hazmat
// Provides: {"impl_serializable_state_u8_array"}
// Dependencies: {}
macro_rules ! impl_serializable_state_u8_array { ($ ($ n : ty) ,*) => { $ (impl SerializableState for [u8 ; <$ n >:: USIZE] { type SerializedStateSize = $ n ; fn serialize (& self) -> SerializedState < Self > { (* self) . into () } fn deserialize (serialized_state : & SerializedState < Self >,) -> Result < Self , DeserializeStateError > { Ok ((* serialized_state) . into ()) } }) * } ; }
};
}
