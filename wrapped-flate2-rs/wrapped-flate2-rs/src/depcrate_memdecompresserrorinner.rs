// Generated macro for DecompressErrorInner (enum)
macro_rules! Depcrate_memDecompressErrorInner {
() => {
// Module: crate::mem
// Provides: {"DecompressErrorInner"}
// Dependencies: {}
# [doc = " The inner state for an error when decompressing"] # [derive (Clone , Debug)] pub (crate) enum DecompressErrorInner { General { msg : ErrorMessage } , NeedsDictionary (u32) , }
};
}
