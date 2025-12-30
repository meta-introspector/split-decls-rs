// Generated macro for GzBuilder (struct)
macro_rules! Depcrate_gzGzBuilder {
() => {
// Module: crate::gz
// Provides: {"GzBuilder"}
// Dependencies: {}
# [doc = " A builder structure to create a new gzip Encoder."] # [doc = ""] # [doc = " This structure controls header configuration options such as the filename."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::prelude::*;"] # [doc = " # use std::io;"] # [doc = " use std::fs::File;"] # [doc = " use flate2::GzBuilder;"] # [doc = " use flate2::Compression;"] # [doc = ""] # [doc = " // GzBuilder opens a file and writes a sample string using GzBuilder pattern"] # [doc = ""] # [doc = " # fn sample_builder() -> Result<(), io::Error> {"] # [doc = " let f = File::create(\"examples/hello_world.gz\")?;"] # [doc = " let mut gz = GzBuilder::new()"] # [doc = "                 .filename(\"hello_world.txt\")"] # [doc = "                 .comment(\"test file, please delete\")"] # [doc = "                 .write(f, Compression::default());"] # [doc = " gz.write_all(b\"hello world\")?;"] # [doc = " gz.finish()?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug , Default)] pub struct GzBuilder { extra : Option < Vec < u8 > > , filename : Option < CString > , comment : Option < CString > , operating_system : Option < u8 > , mtime : u32 , }
};
}
