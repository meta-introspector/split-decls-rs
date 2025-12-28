macro_rules! deps {
    () => {
        Write!();
        Result!();
        FilterConfig!();
    };
}

macro_rules! write_xz_block_header {
    () => {
        deps!();
        # [cfg (feature = "encoder")] fn write_xz_block_header < W : Write > (writer : & mut W , filters : & [FilterConfig] , lzma_dict_size : u32 ,) -> crate :: Result < u64 > { let header_data = generate_block_header_data (filters , lzma_dict_size) ? ; let total_size_needed : usize = 1 + header_data . len () + 4 ; let header_size = total_size_needed . div_ceil (4) * 4 ; let header_size_encoded = ((header_size / 4) - 1) as u8 ; let padding_needed = header_size - 1 - header_data . len () - 4 ; let mut crc = CRC32 . digest () ; crc . update (& [header_size_encoded]) ; crc . update (& header_data) ; update_crc_with_padding (& mut crc , padding_needed) ; let crc_value = crc . finalize () ; writer . write_u8 (header_size_encoded) ? ; writer . write_all (& header_data) ? ; add_padding (writer , padding_needed) ? ; writer . write_u32 (crc_value) ? ; Ok (header_size as u64) }
    };
}

write_xz_block_header!();