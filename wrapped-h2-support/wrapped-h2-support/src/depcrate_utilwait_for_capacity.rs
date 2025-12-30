// Generated macro for wait_for_capacity (function)
macro_rules! Depcrate_utilwait_for_capacity {
() => {
// Module: crate::util
// Provides: {"wait_for_capacity"}
// Dependencies: {}
# [doc = " Should only be called after a non-0 capacity was requested for the stream."] pub fn wait_for_capacity (stream : h2 :: SendStream < Bytes > , target : usize) -> WaitForCapacity { WaitForCapacity { stream : Some (stream) , target , } }
};
}
