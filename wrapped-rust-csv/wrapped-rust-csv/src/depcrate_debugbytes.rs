// Generated macro for Bytes (struct)
macro_rules! Depcrate_debugBytes {
() => {
// Module: crate::debug
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " A type that provides a human readable debug impl for arbitrary bytes."] # [doc = ""] # [doc = " This generally works best when the bytes are presumed to be mostly UTF-8,"] # [doc = " but will work for anything."] # [doc = ""] # [doc = " N.B. This is copied nearly verbatim from regex-automata. Sigh."] pub (crate) struct Bytes < 'a > (pub (crate) & 'a [u8]) ;
};
}
