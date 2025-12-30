// Generated macro for StateType (trait)
macro_rules! Depcrate_lib_oxideStateType {
() => {
// Module: crate::lib_oxide
// Provides: {"StateType"}
// Dependencies: {}
# [doc = " Trait used for states that can be carried by BoxedState."] pub trait StateType { const STATE_TYPE : StateTypeEnum ; fn from_enum (value : & mut InternalState) -> Option < & mut Self > ; }
};
}
