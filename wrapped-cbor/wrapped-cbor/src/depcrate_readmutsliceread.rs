// Generated macro for MutSliceRead (struct)
macro_rules! Depcrate_readMutSliceRead {
() => {
// Module: crate::read
// Provides: {"MutSliceRead"}
// Dependencies: {}
# [doc = " A CBOR input source that reads from a slice of bytes, and can move data around internally to"] # [doc = " reassemble indefinite strings without the need of an allocated scratch buffer."] # [derive (Debug)] pub struct MutSliceRead < 'a > { # [doc = " A complete view of the reader's data. It is promised that bytes before buffer_end are not"] # [doc = " mutated any more."] slice : & 'a mut [u8] , # [doc = " Read cursor position in slice"] index : usize , # [doc = " Number of bytes already discarded from the slice"] before : usize , # [doc = " End of the buffer area that contains all bytes read_into_buffer. This is always <= index."] buffer_end : usize , }
};
}
