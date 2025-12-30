// Generated macro for EmitsChangedSignal (enum)
macro_rules! Depcrate_leavesEmitsChangedSignal {
() => {
// Module: crate::leaves
// Provides: {"EmitsChangedSignal"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , Ord , PartialOrd , Debug)] # [doc = " Enumerates the different signaling behaviors a Property can have"] # [doc = " to being changed."] pub enum EmitsChangedSignal { # [doc = " The Property emits a signal that includes the new value."] True , # [doc = " The Property emits a signal that does not include the new value."] Invalidates , # [doc = " The Property cannot be changed."] Const , # [doc = " The Property does not emit a signal when changed."] False , }
};
}
