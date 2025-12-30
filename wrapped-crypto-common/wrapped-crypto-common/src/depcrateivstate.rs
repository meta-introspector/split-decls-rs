// Generated macro for IvState (trait)
macro_rules! DepcrateIvState {
() => {
// Module: crate
// Provides: {"IvState"}
// Dependencies: {}
# [doc = " Trait for loading current IV state."] pub trait IvState : IvSizeUser { # [doc = " Returns current IV state."] fn iv_state (& self) -> Iv < Self > ; }
};
}
