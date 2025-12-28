macro_rules! write_file_header {
    () => {
        pub (crate) fn write_file_header (stream : & mut FileEncoder , sess : & Session) { stream . emit_raw_bytes (FILE_MAGIC) ; stream . emit_raw_bytes (& [(HEADER_FORMAT_VERSION >> 0) as u8 , (HEADER_FORMAT_VERSION >> 8) as u8]) ; let rustc_version = rustc_version (sess . is_nightly_build () , sess . cfg_version) ; assert_eq ! (rustc_version . len () , (rustc_version . len () as u8) as usize) ; stream . emit_raw_bytes (& [rustc_version . len () as u8]) ; stream . emit_raw_bytes (rustc_version . as_bytes ()) ; }
    };
}

write_file_header!();