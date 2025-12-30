// Generated macro for Assembler (struct)
macro_rules! Depcrate_connection_assemblerAssembler {
() => {
// Module: crate::connection::assembler
// Provides: {"Assembler"}
// Dependencies: {}
# [doc = " Helper to assemble unordered stream frames into an ordered stream"] # [derive (Debug , Default)] pub (super) struct Assembler { state : State , data : BinaryHeap < Buffer > , # [doc = " Total number of buffered bytes, including duplicates in ordered mode."] buffered : usize , # [doc = " Estimated number of allocated bytes, will never be less than `buffered`."] allocated : usize , # [doc = " Number of bytes read by the application. When only ordered reads have been used, this is the"] # [doc = " length of the contiguous prefix of the stream which has been consumed by the application,"] # [doc = " aka the stream offset."] bytes_read : u64 , end : u64 , }
};
}
