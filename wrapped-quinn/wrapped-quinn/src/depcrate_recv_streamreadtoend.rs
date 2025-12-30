// Generated macro for ReadToEnd (struct)
macro_rules! Depcrate_recv_streamReadToEnd {
() => {
// Module: crate::recv_stream
// Provides: {"ReadToEnd"}
// Dependencies: {}
# [doc = " Future produced by [`RecvStream::read_to_end()`]."] # [doc = ""] # [doc = " [`RecvStream::read_to_end()`]: crate::RecvStream::read_to_end"] struct ReadToEnd < 'a > { stream : & 'a mut RecvStream , read : Vec < (Bytes , u64) > , start : u64 , end : u64 , size_limit : usize , }
};
}
