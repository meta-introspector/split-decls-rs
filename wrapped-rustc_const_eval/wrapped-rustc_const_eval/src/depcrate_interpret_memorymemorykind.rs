// Generated macro for MemoryKind (enum)
macro_rules! Depcrate_interpret_memoryMemoryKind {
() => {
// Module: crate::interpret::memory
// Provides: {"MemoryKind"}
// Dependencies: {}
# [derive (Debug , PartialEq , Copy , Clone)] pub enum MemoryKind < T > { # [doc = " Stack memory. Error if deallocated except during a stack pop."] Stack , # [doc = " Memory allocated by `caller_location` intrinsic. Error if ever deallocated."] CallerLocation , # [doc = " Additional memory kinds a machine wishes to distinguish from the builtin ones."] Machine (T) , }
};
}
