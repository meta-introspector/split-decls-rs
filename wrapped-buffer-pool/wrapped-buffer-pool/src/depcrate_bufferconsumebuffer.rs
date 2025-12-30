// Generated macro for ConsumeBuffer (struct)
macro_rules! Depcrate_bufferConsumeBuffer {
() => {
// Module: crate::buffer
// Provides: {"ConsumeBuffer"}
// Dependencies: {}
# [doc = " A convinience wrapper around Vec that allows to \"consume\" data from the"] # [doc = " front *without* shifting."] # [doc = ""] # [doc = " This is not unlike `VecDeque` but more ergonomic"] # [doc = " for the operations we require. Conceptually `VecDeque` is two slices, and"] # [doc = " this is one slice. Also there is no `set_len` for `VecDeque`, so it has to"] # [doc = " be converted to `Vec` and then back again."] # [derive (Default , Debug)] pub struct ConsumeBuffer { inner : Vec < u8 > , head : usize , }
};
}
