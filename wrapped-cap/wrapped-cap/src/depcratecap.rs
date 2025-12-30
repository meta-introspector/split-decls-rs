// Generated macro for Cap (struct)
macro_rules! DepcrateCap {
() => {
// Module: crate
// Provides: {"Cap"}
// Dependencies: {}
# [doc = " A struct that wraps another allocator and limits the number of bytes that can be allocated."] # [derive (Debug)] pub struct Cap < H > { allocator : H , remaining : AtomicUsize , limit : AtomicUsize , # [cfg (feature = "stats")] total_allocated : AtomicUsize , # [cfg (feature = "stats")] max_allocated : AtomicUsize , }
};
}
