// Generated macro for DeflateEncoder (struct)
macro_rules! Depcrate_deflate_readDeflateEncoder {
() => {
// Module: crate::deflate::read
// Provides: {"DeflateEncoder"}
// Dependencies: {}
# [doc = " A DEFLATE encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a [`Read`] interface. When read from, it reads"] # [doc = " uncompressed data from the underlying [`Read`] and provides the compressed data."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use std::io;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::read::DeflateEncoder;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " #    println!(\"{:?}\", deflateencoder_read_hello_world().unwrap());"] # [doc = " # }"] # [doc = " #"] # [doc = " // Return a vector containing the Deflate compressed version of hello world"] # [doc = " fn deflateencoder_read_hello_world() -> io::Result<Vec<u8>> {"] # [doc = "    let mut ret_vec = Vec::new();"] # [doc = "    let c = b\"hello world\";"] # [doc = "    let mut deflater = DeflateEncoder::new(&c[..], Compression::fast());"] # [doc = "    deflater.read_to_end(&mut ret_vec)?;"] # [doc = "    Ok(ret_vec)"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct DeflateEncoder < R > { inner : bufread :: DeflateEncoder < BufReader < R > > , }
};
}
