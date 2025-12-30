// Generated macro for InstPtr (type)
macro_rules! Depcrate_dfaInstPtr {
() => {
// Module: crate::dfa
// Provides: {"InstPtr"}
// Dependencies: {}
# [doc = " InstPtr is a 32 bit pointer into a sequence of opcodes (i.e., it indexes"] # [doc = " an NFA state)."] # [doc = ""] # [doc = " Throughout this library, this is usually set to `usize`, but we force a"] # [doc = " `u32` here for the DFA to save on space."] type InstPtr = u32 ;
};
}
