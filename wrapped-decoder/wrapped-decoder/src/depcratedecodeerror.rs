// Generated macro for DecodeError (enum)
macro_rules! DepcrateDecodeError {
() => {
// Module: crate
// Provides: {"DecodeError"}
// Dependencies: {}
# [doc = " Ways in which decoding a defmt frame can fail."] # [derive (Debug , Eq , PartialEq)] pub enum DecodeError { # [doc = " More data is needed to decode the next frame."] UnexpectedEof , # [doc = " The frame was not in the expected format."] Malformed , }
};
}
