// Generated macro for Decompress (struct)
macro_rules! Depcrate_memDecompress {
() => {
// Module: crate::mem
// Provides: {"Decompress"}
// Dependencies: {}
# [doc = " Raw in-memory decompression stream for blocks of data."] # [doc = ""] # [doc = " This type is the building block for the I/O streams in the rest of this"] # [doc = " crate. It requires more management than the [`Read`]/[`Write`] API but is"] # [doc = " maximally flexible in terms of accepting input from any source and being"] # [doc = " able to produce output to any memory location."] # [doc = ""] # [doc = " It is recommended to use the I/O stream adaptors over this type as they're"] # [doc = " easier to use."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = " [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"] # [derive (Debug)] pub struct Decompress { inner : Inflate , }
};
}
