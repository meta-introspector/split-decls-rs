// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/rpc.rs
// Error: expected square brackets
// Problematic line: line 10


pub(super) type Writer = super::buffer::Buffer;

pub(super) trait Encode<S>: Sized {
    fn encode(self, w: &mut Writer, s: &mut S);
}

