// Generated macro for DeflateEncoder (struct)
macro_rules! Depcrate_deflate_writeDeflateEncoder {
() => {
// Module: crate::deflate::write
// Provides: {"DeflateEncoder"}
// Dependencies: {}
# [doc = " A DEFLATE encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a [`Write`] interface and takes a stream of"] # [doc = " uncompressed data, writing the compressed data to the wrapped writer."] # [doc = ""] # [doc = " [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::write::DeflateEncoder;"] # [doc = ""] # [doc = " // Vec<u8> implements Write to print the compressed bytes of sample string"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut e = DeflateEncoder::new(Vec::new(), Compression::default());"] # [doc = " e.write_all(b\"Hello World\").unwrap();"] # [doc = " println!(\"{:?}\", e.finish().unwrap());"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct DeflateEncoder < W : Write > { inner : zio :: Writer < W , Compress > , }
};
}
