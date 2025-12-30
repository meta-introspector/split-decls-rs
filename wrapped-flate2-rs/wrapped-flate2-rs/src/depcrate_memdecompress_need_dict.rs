// Generated macro for decompress_need_dict (function)
macro_rules! Depcrate_memdecompress_need_dict {
() => {
// Module: crate::mem
// Provides: {"decompress_need_dict"}
// Dependencies: {}
# [inline] pub (crate) fn decompress_need_dict < T > (adler : u32) -> Result < T , DecompressError > { Err (DecompressError (DecompressErrorInner :: NeedsDictionary (adler ,))) }
};
}
