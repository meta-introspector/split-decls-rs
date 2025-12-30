// Generated macro for GzEncoder (struct)
macro_rules! Depcrate_gz_bufreadGzEncoder {
() => {
// Module: crate::gz::bufread
// Provides: {"GzEncoder"}
// Dependencies: {}
# [doc = " A gzip streaming encoder"] # [doc = ""] # [doc = " This structure implements a [`Read`] interface. When read from, it reads"] # [doc = " uncompressed data from the underlying [`BufRead`] and provides the compressed data."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = " [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use std::io;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::bufread::GzEncoder;"] # [doc = " use std::fs::File;"] # [doc = " use std::io::BufReader;"] # [doc = ""] # [doc = " // Opens sample file, compresses the contents and returns a Vector or error"] # [doc = " // File wrapped in a BufReader implements BufRead"] # [doc = ""] # [doc = " fn open_hello_world() -> io::Result<Vec<u8>> {"] # [doc = "     let f = File::open(\"examples/hello_world.txt\")?;"] # [doc = "     let b = BufReader::new(f);"] # [doc = "     let mut gz = GzEncoder::new(b, Compression::fast());"] # [doc = "     let mut buffer = Vec::new();"] # [doc = "     gz.read_to_end(&mut buffer)?;"] # [doc = "     Ok(buffer)"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct GzEncoder < R > { inner : deflate :: bufread :: DeflateEncoder < CrcReader < R > > , header : Vec < u8 > , pos : usize , eof : bool , }
};
}
