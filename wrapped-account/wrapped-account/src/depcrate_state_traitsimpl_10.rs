// Generated macro for impl_10 (impl)
macro_rules! Depcrate_state_traitsimpl_10 {
() => {
// Module: crate::state_traits
// Provides: {"impl_10"}
// Dependencies: {}
impl < T > StateMut < T > for Account where T : serde :: Serialize + serde :: de :: DeserializeOwned , { fn state (& self) -> Result < T , InstructionError > { self . deserialize_data () . map_err (| _ | InstructionError :: InvalidAccountData) } fn set_state (& mut self , state : & T) -> Result < () , InstructionError > { self . serialize_data (state) . map_err (| err | match * err { ErrorKind :: SizeLimit => InstructionError :: AccountDataTooSmall , _ => InstructionError :: GenericError , }) } }
};
}
