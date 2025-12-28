macro_rules! deps {
    () => {
        IndexRecord!();
        Result!();
        Write!();
    };
}

macro_rules! write_xz_index {
    () => {
        deps!();
        # [cfg (feature = "encoder")] fn write_xz_index < W : Write > (writer : & mut W , index_records : & [IndexRecord]) -> crate :: Result < () > { let mut index_data = Vec :: new () ; let mut temp_buf = [0u8 ; 10] ; let size = encode_multibyte_integer (index_records . len () as u64 , & mut temp_buf) ? ; index_data . extend_from_slice (& temp_buf [.. size]) ; for record in index_records { let size = encode_multibyte_integer (record . unpadded_size , & mut temp_buf) ? ; index_data . extend_from_slice (& temp_buf [.. size]) ; let size = encode_multibyte_integer (record . uncompressed_size , & mut temp_buf) ? ; index_data . extend_from_slice (& temp_buf [.. size]) ; } let bytes_written = 1 + index_data . len () ; let padding_needed = (4 - (bytes_written % 4)) % 4 ; let mut crc = CRC32 . digest () ; crc . update (& [0x00]) ; crc . update (& index_data) ; update_crc_with_padding (& mut crc , padding_needed) ; let crc_value = crc . finalize () ; writer . write_u8 (0x00) ? ; writer . write_all (& index_data) ? ; add_padding (writer , padding_needed) ? ; writer . write_u32 (crc_value) ? ; Ok (()) }
    };
}

write_xz_index!();