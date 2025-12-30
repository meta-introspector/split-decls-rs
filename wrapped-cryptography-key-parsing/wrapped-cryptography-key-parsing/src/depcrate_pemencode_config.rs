// Generated macro for ENCODE_CONFIG (const)
macro_rules! Depcrate_pemENCODE_CONFIG {
() => {
// Module: crate::pem
// Provides: {"ENCODE_CONFIG"}
// Dependencies: {}
# [doc = " PEM encoding configuration using LF line endings (Unix-style)"] pub const ENCODE_CONFIG : pem :: EncodeConfig = pem :: EncodeConfig :: new () . set_line_ending (pem :: LineEnding :: LF) ;
};
}
