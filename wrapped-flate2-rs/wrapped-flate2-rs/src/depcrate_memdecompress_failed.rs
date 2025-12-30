// Generated macro for decompress_failed (function)
macro_rules! Depcrate_memdecompress_failed {
() => {
// Module: crate::mem
// Provides: {"decompress_failed"}
// Dependencies: {}
# [inline] pub (crate) fn decompress_failed < T > (msg : ErrorMessage) -> Result < T , DecompressError > { Err (DecompressError (DecompressErrorInner :: General { msg })) }
};
}
