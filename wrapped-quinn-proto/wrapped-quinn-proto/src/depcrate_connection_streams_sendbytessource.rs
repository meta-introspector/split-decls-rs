// Generated macro for BytesSource (trait)
macro_rules! Depcrate_connection_streams_sendBytesSource {
() => {
// Module: crate::connection::streams::send
// Provides: {"BytesSource"}
// Dependencies: {}
# [doc = " A source of one or more buffers which can be converted into `Bytes` buffers on demand"] # [doc = ""] # [doc = " The purpose of this data type is to defer conversion as long as possible,"] # [doc = " so that no heap allocation is required in case no data is writable."] pub (super) trait BytesSource { # [doc = " Returns the next chunk from the source of owned chunks."] # [doc = ""] # [doc = " This method will consume parts of the source."] # [doc = " Calling it will yield `Bytes` elements up to the configured `limit`."] # [doc = ""] # [doc = " The method returns a tuple:"] # [doc = " - The first item is the yielded `Bytes` element. The element will be"] # [doc = "   empty if the limit is zero or no more data is available."] # [doc = " - The second item returns how many complete chunks inside the source had"] # [doc = "   had been consumed. This can be less than 1, if a chunk inside the"] # [doc = "   source had been truncated in order to adhere to the limit. It can also"] # [doc = "   be more than 1, if zero-length chunks had been skipped."] fn pop_chunk (& mut self , limit : usize) -> (Bytes , usize) ; }
};
}
