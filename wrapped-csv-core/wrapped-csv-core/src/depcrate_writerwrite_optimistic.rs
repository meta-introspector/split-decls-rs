// Generated macro for write_optimistic (function)
macro_rules! Depcrate_writerwrite_optimistic {
() => {
// Module: crate::writer
// Provides: {"write_optimistic"}
// Dependencies: {}
# [doc = " Copy the bytes from `input` to `output`. If `output` is too small to fit"] # [doc = " everything from `input`, then copy `output.len()` bytes from `input`."] # [doc = " Otherwise, copy everything from `input` into `output`."] # [doc = ""] # [doc = " In the first case (`output` is too small), `WriteResult::OutputFull` is"] # [doc = " returned, in addition to the number of bytes consumed from `input` and"] # [doc = " the number of bytes written to `output`."] # [doc = ""] # [doc = " In the second case (`input` is no bigger than `output`),"] # [doc = " `WriteResult::InputEmpty` is returned, in addition to the number of bytes"] # [doc = " consumed from `input` and the number of bytes written to `output`."] fn write_optimistic (input : & [u8] , output : & mut [u8] ,) -> (WriteResult , usize , usize) { if input . len () > output . len () { let input = & input [.. output . len ()] ; output . copy_from_slice (input) ; (WriteResult :: OutputFull , output . len () , output . len ()) } else { output [.. input . len ()] . copy_from_slice (input) ; (WriteResult :: InputEmpty , input . len () , input . len ()) } }
};
}
