// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_vec_dequeDrain {
() => {
// Module: crate::collections::vec_deque
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " Draining parallel iterator that moves a range out of a double-ended queue,"] # [doc = " but keeps the total capacity."] # [derive (Debug)] pub struct Drain < 'a , T > { deque : & 'a mut VecDeque < T > , range : Range < usize > , orig_len : usize , }
};
}
