// Generated macro for DeflateEncoder (struct)
macro_rules! Depcrate_deflate_bufreadDeflateEncoder {
() => {
// Module: crate::deflate::bufread
// Provides: {"DeflateEncoder"}
// Dependencies: {}
# [doc = " A DEFLATE encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a [`Read`] interface. When read from, it reads"] # [doc = " uncompressed data from the underlying [`BufRead`] and provides the compressed data."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = " [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use std::io;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::bufread::DeflateEncoder;"] # [doc = " use std::fs::File;"] # [doc = " use std::io::BufReader;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " #    println!(\"{:?}\", open_hello_world().unwrap());"] # [doc = " # }"] # [doc = " #"] # [doc = " // Opens sample file, compresses the contents and returns a Vector"] # [doc = " fn open_hello_world() -> io::Result<Vec<u8>> {"] # [doc = "    let f = File::open(\"examples/hello_world.txt\")?;"] # [doc = "    let b = BufReader::new(f);"] # [doc = "    let mut deflater = DeflateEncoder::new(b, Compression::fast());"] # [doc = "    let mut buffer = Vec::new();"] # [doc = "    deflater.read_to_end(&mut buffer)?;"] # [doc = "    Ok(buffer)"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct DeflateEncoder < R > { obj : R , data : Compress , }
};
}
