// Generated macro for impl_269 (impl)
macro_rules! Depcrate_memimpl_269 {
() => {
// Module: crate::mem
// Provides: {"impl_269"}
// Dependencies: {}
impl DecompressError { # [doc = " Indicates whether decompression failed due to requiring a dictionary."] # [doc = ""] # [doc = " The resulting integer is the Adler-32 checksum of the dictionary"] # [doc = " required."] pub fn needs_dictionary (& self) -> Option < u32 > { match self . 0 { DecompressErrorInner :: NeedsDictionary (adler) => Some (adler) , _ => None , } } }
};
}
