// Generated macro for GzEncoder (struct)
macro_rules! Depcrate_gz_readGzEncoder {
() => {
// Module: crate::gz::read
// Provides: {"GzEncoder"}
// Dependencies: {}
# [doc = " A gzip streaming encoder"] # [doc = ""] # [doc = " This structure implements a [`Read`] interface. When read from, it reads"] # [doc = " uncompressed data from the underlying [`Read`] and provides the compressed data."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " use std::io;"] # [doc = " use flate2::Compression;"] # [doc = " use flate2::read::GzEncoder;"] # [doc = ""] # [doc = " // Return a vector containing the GZ compressed version of hello world"] # [doc = ""] # [doc = " fn gzencode_hello_world() -> io::Result<Vec<u8>> {"] # [doc = "     let mut ret_vec = Vec::new();"] # [doc = "     let bytestring = b\"hello world\";"] # [doc = "     let mut gz = GzEncoder::new(&bytestring[..], Compression::fast());"] # [doc = "     gz.read_to_end(&mut ret_vec)?;"] # [doc = "     Ok(ret_vec)"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct GzEncoder < R > { inner : bufread :: GzEncoder < BufReader < R > > , }
};
}
