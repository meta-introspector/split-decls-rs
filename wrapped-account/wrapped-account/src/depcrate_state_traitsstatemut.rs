// Generated macro for StateMut (trait)
macro_rules! Depcrate_state_traitsStateMut {
() => {
// Module: crate::state_traits
// Provides: {"StateMut"}
// Dependencies: {}
# [doc = " Convenience trait to covert bincode errors to instruction errors."] pub trait StateMut < T > { fn state (& self) -> Result < T , InstructionError > ; fn set_state (& mut self , state : & T) -> Result < () , InstructionError > ; }
};
}
