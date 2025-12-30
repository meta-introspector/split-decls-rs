// Generated macro for Bytes (struct)
macro_rules! Depcrate_shared_util_escapeBytes {
() => {
// Module: crate::shared::util::escape
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " Provides a convenient `Debug` implementation for `&[u8]`."] # [doc = ""] # [doc = " This generally works best when the bytes are presumed to be mostly"] # [doc = " UTF-8, but will work for anything. For any bytes that aren't UTF-8,"] # [doc = " they are emitted as hex escape sequences."] # [derive (Clone , Copy)] pub (crate) struct Bytes < 'a > (pub & 'a [u8]) ;
};
}
