// Generated macro for ZlibEncoder (struct)
macro_rules! Depcrate_zlib_bufreadZlibEncoder {
() => {
// Module: crate::zlib::bufread
// Provides: {"ZlibEncoder"}
// Dependencies: {}
# [doc = " A ZLIB encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a [`Read`] interface. When read from, it reads"] # [doc = " uncompressed data from the underlying [`BufRead`] and provides the compressed data."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = " [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::bufread::ZlibEncoder;"] # [doc = " use std::fs::File;"] # [doc = " use std::io::BufReader;"] # [doc = ""] # [doc = " // Use a buffered file to compress contents into a Vec<u8>"] # [doc = ""] # [doc = " # fn open_hello_world() -> std::io::Result<Vec<u8>> {"] # [doc = " let f = File::open(\"examples/hello_world.txt\")?;"] # [doc = " let b = BufReader::new(f);"] # [doc = " let mut z = ZlibEncoder::new(b, Compression::fast());"] # [doc = " let mut buffer = Vec::new();"] # [doc = " z.read_to_end(&mut buffer)?;"] # [doc = " # Ok(buffer)"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct ZlibEncoder < R > { obj : R , data : Compress , }
};
}
