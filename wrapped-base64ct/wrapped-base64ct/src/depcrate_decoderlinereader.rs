// Generated macro for LineReader (struct)
macro_rules! Depcrate_decoderLineReader {
() => {
// Module: crate::decoder
// Provides: {"LineReader"}
// Dependencies: {}
# [doc = " Iterator over multi-line Base64 input."] # [derive (Clone)] struct LineReader < 'i > { # [doc = " Remaining linewrapped data to be processed."] remaining : & 'i [u8] , # [doc = " Line width."] line_width : Option < usize > , }
};
}
