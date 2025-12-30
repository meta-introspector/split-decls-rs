// Generated macro for impl_12 (impl)
macro_rules! Depcrate_state_traitsimpl_12 {
() => {
// Module: crate::state_traits
// Provides: {"impl_12"}
// Dependencies: {}
impl < T > StateMut < T > for Ref < '_ , AccountSharedData > where T : serde :: Serialize + serde :: de :: DeserializeOwned , { fn state (& self) -> Result < T , InstructionError > { self . deserialize_data () . map_err (| _ | InstructionError :: InvalidAccountData) } fn set_state (& mut self , _state : & T) -> Result < () , InstructionError > { panic ! ("illegal") ; } }
};
}
