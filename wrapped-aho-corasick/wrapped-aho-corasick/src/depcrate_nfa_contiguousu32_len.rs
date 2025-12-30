// Generated macro for u32_len (function)
macro_rules! Depcrate_nfa_contiguousu32_len {
() => {
// Module: crate::nfa::contiguous
// Provides: {"u32_len"}
// Dependencies: {}
# [doc = " Computes the number of u32 values needed to represent one byte per the"] # [doc = " number of transitions given."] fn u32_len (ntrans : usize) -> usize { if ntrans % 4 == 0 { ntrans >> 2 } else { (ntrans >> 2) + 1 } }
};
}
