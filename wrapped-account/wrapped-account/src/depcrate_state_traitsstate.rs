// Generated macro for State (trait)
macro_rules! Depcrate_state_traitsState {
() => {
// Module: crate::state_traits
// Provides: {"State"}
// Dependencies: {}
pub trait State < T > { fn state (& self) -> Result < T , InstructionError > ; fn set_state (& self , state : & T) -> Result < () , InstructionError > ; }
};
}
