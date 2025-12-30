// Generated macro for ZlibEncoder (struct)
macro_rules! Depcrate_zlib_writeZlibEncoder {
() => {
// Module: crate::zlib::write
// Provides: {"ZlibEncoder"}
// Dependencies: {}
# [doc = " A ZLIB encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a [`Write`] interface and takes a stream of"] # [doc = " uncompressed data, writing the compressed data to the wrapped writer."] # [doc = ""] # [doc = " [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::write::ZlibEncoder;"] # [doc = ""] # [doc = " // Vec<u8> implements Write, assigning the compressed bytes of sample string"] # [doc = ""] # [doc = " # fn zlib_encoding() -> std::io::Result<()> {"] # [doc = " let mut e = ZlibEncoder::new(Vec::new(), Compression::default());"] # [doc = " e.write_all(b\"Hello World\")?;"] # [doc = " let compressed = e.finish()?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct ZlibEncoder < W : Write > { inner : zio :: Writer < W , Compress > , }
};
}
