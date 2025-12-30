// Generated macro for GzEncoder (struct)
macro_rules! Depcrate_gz_writeGzEncoder {
() => {
// Module: crate::gz::write
// Provides: {"GzEncoder"}
// Dependencies: {}
# [doc = " A gzip streaming encoder"] # [doc = ""] # [doc = " This structure exposes a [`Write`] interface that will emit compressed data"] # [doc = " to the underlying writer `W`."] # [doc = ""] # [doc = " [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::write::GzEncoder;"] # [doc = ""] # [doc = " // Vec<u8> implements Write to print the compressed bytes of sample string"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut e = GzEncoder::new(Vec::new(), Compression::default());"] # [doc = " e.write_all(b\"Hello World\").unwrap();"] # [doc = " println!(\"{:?}\", e.finish().unwrap());"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct GzEncoder < W : Write > { inner : zio :: Writer < W , Compress > , crc : Crc , crc_bytes_written : usize , header : Vec < u8 > , }
};
}
