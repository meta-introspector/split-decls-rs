macro_rules! write_file_header {
    () => {
        pub fn write_file_header (s : & mut dyn std :: io :: Write , file_magic : & [u8 ; 4] ,) -> Result < () , Box < dyn Error + Send + Sync > > { assert_eq ! (FILE_HEADER_SIZE , 8) ; s . write_all (file_magic) . map_err (Box :: new) ? ; s . write_all (& CURRENT_FILE_FORMAT_VERSION . to_le_bytes ()) . map_err (Box :: new) ? ; Ok (()) }
    };
}

write_file_header!()