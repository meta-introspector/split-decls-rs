// Generated macro for write_pessimistic (function)
macro_rules! Depcrate_writerwrite_pessimistic {
() => {
// Module: crate::writer
// Provides: {"write_pessimistic"}
// Dependencies: {}
# [doc = " Copy the bytes from `input` to `output` only if `input` is no bigger than"] # [doc = " `output`. If `input` is bigger than `output`, then return"] # [doc = " `WriteResult::OutputFull` and copy nothing into `output`. Otherwise,"] # [doc = " return `WriteResult::InputEmpty` and the number of bytes copied into"] # [doc = " `output`."] fn write_pessimistic (input : & [u8] , output : & mut [u8]) -> (WriteResult , usize) { if input . len () > output . len () { (WriteResult :: OutputFull , 0) } else { output [.. input . len ()] . copy_from_slice (input) ; (WriteResult :: InputEmpty , input . len ()) } }
};
}
