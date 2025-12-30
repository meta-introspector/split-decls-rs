// Generated macro for ZlibEncoder (struct)
macro_rules! Depcrate_zlib_readZlibEncoder {
() => {
// Module: crate::zlib::read
// Provides: {"ZlibEncoder"}
// Dependencies: {}
# [doc = " A ZLIB encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a [`Read`] interface. When read from, it reads"] # [doc = " uncompressed data from the underlying [`Read`] and provides the compressed data."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::read::ZlibEncoder;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " // Open example file and compress the contents using Read interface"] # [doc = ""] # [doc = " # fn open_hello_world() -> std::io::Result<Vec<u8>> {"] # [doc = " let f = File::open(\"examples/hello_world.txt\")?;"] # [doc = " let mut z = ZlibEncoder::new(f, Compression::fast());"] # [doc = " let mut buffer = Vec::new();"] # [doc = " z.read_to_end(&mut buffer)?;"] # [doc = " # Ok(buffer)"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct ZlibEncoder < R > { inner : bufread :: ZlibEncoder < BufReader < R > > , }
};
}
