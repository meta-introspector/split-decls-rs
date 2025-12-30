// Generated macro for read (module)
macro_rules! Depcrateread {
() => {
// Module: crate
// Provides: {"read"}
// Dependencies: {}
# [doc = " Types which operate over [`Read`] streams, both encoders and decoders for"] # [doc = " various formats."] # [doc = ""] # [doc = " Note that the `read` decoder types may read past the end of the compressed"] # [doc = " data while decoding. If the caller requires subsequent reads to start"] # [doc = " immediately following the compressed data  wrap the `Read` type in a"] # [doc = " [`BufReader`] and use the `BufReader` with the equivalent decoder from the"] # [doc = " `bufread` module and also for the subsequent reads."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = " [`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html"] pub mod read { pub use crate :: deflate :: read :: DeflateDecoder ; pub use crate :: deflate :: read :: DeflateEncoder ; pub use crate :: gz :: read :: GzDecoder ; pub use crate :: gz :: read :: GzEncoder ; pub use crate :: gz :: read :: MultiGzDecoder ; pub use crate :: zlib :: read :: ZlibDecoder ; pub use crate :: zlib :: read :: ZlibEncoder ; }
};
}
