// Generated macro for Drain (struct)
macro_rules! Depcrate_vecDrain {
() => {
// Module: crate::vec
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " Draining parallel iterator that moves a range out of a vector, but keeps the total capacity."] # [derive (Debug)] pub struct Drain < 'data , T : Send > { vec : & 'data mut Vec < T > , range : Range < usize > , orig_len : usize , }
};
}
