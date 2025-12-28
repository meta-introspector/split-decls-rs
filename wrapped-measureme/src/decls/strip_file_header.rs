macro_rules! strip_file_header {
    () => {
        pub fn strip_file_header (data : & [u8]) -> & [u8] { & data [FILE_HEADER_SIZE ..] }
    };
}

strip_file_header!();