// Generated macro for bufread (module)
macro_rules! Depcratebufread {
() => {
// Module: crate
// Provides: {"bufread"}
// Dependencies: {}
# [doc = " Types which operate over [`BufRead`] streams, both encoders and decoders for"] # [doc = " various formats."] # [doc = ""] # [doc = " [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html"] pub mod bufread { pub use crate :: deflate :: bufread :: DeflateDecoder ; pub use crate :: deflate :: bufread :: DeflateEncoder ; pub use crate :: gz :: bufread :: GzDecoder ; pub use crate :: gz :: bufread :: GzEncoder ; pub use crate :: gz :: bufread :: MultiGzDecoder ; pub use crate :: zlib :: bufread :: ZlibDecoder ; pub use crate :: zlib :: bufread :: ZlibEncoder ; }
};
}
