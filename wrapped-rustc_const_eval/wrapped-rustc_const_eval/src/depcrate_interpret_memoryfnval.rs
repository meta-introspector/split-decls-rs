// Generated macro for FnVal (enum)
macro_rules! Depcrate_interpret_memoryFnVal {
() => {
// Module: crate::interpret::memory
// Provides: {"FnVal"}
// Dependencies: {}
# [doc = " The value of a function pointer."] # [derive (Debug , Copy , Clone)] pub enum FnVal < 'tcx , Other > { Instance (Instance < 'tcx >) , Other (Other) , }
};
}
