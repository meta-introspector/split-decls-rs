// Generated macro for Threads (struct)
macro_rules! Depcrate_pikevmThreads {
() => {
// Module: crate::pikevm
// Provides: {"Threads"}
// Dependencies: {}
# [doc = " An ordered set of NFA states and their captures."] # [derive (Clone , Debug)] struct Threads { # [doc = " An ordered set of opcodes (each opcode is an NFA state)."] set : SparseSet , # [doc = " Captures for every NFA state."] # [doc = ""] # [doc = " It is stored in row-major order, where the columns are the capture"] # [doc = " slots and the rows are the states."] caps : Vec < Slot > , # [doc = " The number of capture slots stored per thread. (Every capture has"] # [doc = " two slots.)"] slots_per_thread : usize , }
};
}
