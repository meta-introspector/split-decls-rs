// Generated macro for write (module)
macro_rules! Depcratewrite {
() => {
// Module: crate
// Provides: {"write"}
// Dependencies: {}
# [doc = " Types which operate over [`Write`] streams, both encoders and decoders for"] # [doc = " various formats."] # [doc = ""] # [doc = " [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"] pub mod write { pub use crate :: deflate :: write :: DeflateDecoder ; pub use crate :: deflate :: write :: DeflateEncoder ; pub use crate :: gz :: write :: GzDecoder ; pub use crate :: gz :: write :: GzEncoder ; pub use crate :: gz :: write :: MultiGzDecoder ; pub use crate :: zlib :: write :: ZlibDecoder ; pub use crate :: zlib :: write :: ZlibEncoder ; }
};
}
