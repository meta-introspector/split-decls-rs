// Generated macro for MemoryKind (enum)
macro_rules! Depcrate_const_eval_machineMemoryKind {
() => {
// Module: crate::const_eval::machine
// Provides: {"MemoryKind"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum MemoryKind { Heap { # [doc = " Indicates whether `make_global` was called on this allocation."] # [doc = " If this is `true`, the allocation must be immutable."] was_made_global : bool , } , }
};
}
