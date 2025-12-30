// Generated macro for Stream (struct)
macro_rules! Depcrate_stream_bufferedStream {
() => {
// Module: crate::stream::buffered
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " `Stream` which buffers items from an instance of `StreamOnce` into a ring buffer."] # [doc = " Instances of `StreamOnce` which is not able to implement `ResetStream` (such as `ReadStream`) may"] # [doc = " use this as a way to implement `ResetStream` and become a full `Stream` instance."] # [doc = ""] # [doc = " The drawback is that the buffer only stores a limited number of items which limits how many"] # [doc = " tokens that can be reset and replayed. If a `buffered::Stream` is reset past this limit an error"] # [doc = " will be returned when `uncons` is next called."] # [doc = ""] # [doc = " NOTE: If this stream is used in conjunction with an error enhancing stream such as"] # [doc = " `easy::Stream` (also via the `easy_parser` method) it is recommended that the `buffered::Stream`"] # [doc = " instance wraps the `easy::Stream` instance instead of the other way around."] # [doc = ""] # [doc = " ```ignore"] # [doc = " // DO"] # [doc = " buffered::Stream::new(easy::Stream(..), ..)"] # [doc = " // DON'T"] # [doc = " easy::Stream(buffered::Stream::new(.., ..))"] # [doc = " parser.easy_parse(buffered::Stream::new(..));"] # [doc = " ```"] # [derive (Debug , PartialEq)] pub struct Stream < Input > where Input : StreamOnce + Positioned , { offset : usize , iter : Input , buffer_offset : usize , buffer : VecDeque < (Input :: Token , Input :: Position) > , }
};
}
