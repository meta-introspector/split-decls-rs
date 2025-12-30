// Generated macro for AsciiReader (struct)
macro_rules! Depcrate_stream_ascii_readerAsciiReader {
() => {
// Module: crate::stream::ascii_reader
// Provides: {"AsciiReader"}
// Dependencies: {}
pub struct AsciiReader < R : Read > { reader : R , current_pos : u64 , # [doc = " lookahead char to avoid backtracking."] peeked_char : Option < u8 > , current_char : Option < u8 > , }
};
}
