// Generated macro for impl_150 (impl)
macro_rules! Depcrate_stream_bufferedimpl_150 {
() => {
// Module: crate::stream::buffered
// Provides: {"impl_150"}
// Dependencies: {}
impl < Input > Stream < Input > where Input : StreamOnce + Positioned , Input :: Position : Clone , Input :: Token : Clone , { # [doc = " Constructs a new `BufferedStream` from a `StreamOnce` instance with a `lookahead`"] # [doc = " number of elements that can be stored in the buffer."] pub fn new (iter : Input , lookahead : usize) -> Stream < Input > { Stream { offset : 0 , iter , buffer_offset : 0 , buffer : VecDeque :: with_capacity (lookahead) , } } }
};
}
