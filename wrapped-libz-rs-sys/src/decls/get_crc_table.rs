macro_rules! get_crc_table {
    () => {
        # [doc = " The CRC table used by the crc32 checksum algorithm."] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (get_crc_table))] pub extern "C" fn get_crc_table () -> * const [u32 ; 256] { zlib_rs :: get_crc_table () }
    };
}

get_crc_table!()