macro_rules! deps {
    () => {
        IndexRecord!();
        CheckType!();
        Write!();
        Result!();
    };
}

macro_rules! write_xz_stream_footer {
    () => {
        deps!();
        # [cfg (feature = "encoder")] fn write_xz_stream_footer < W : Write > (writer : & mut W , index_records : & [IndexRecord] , check_type : CheckType ,) -> crate :: Result < () > { let mut index_size = 1 ; index_size += count_multibyte_integer_size_for_value (index_records . len () as u64) ; for record in index_records { index_size += count_multibyte_integer_size_for_value (record . unpadded_size) ; index_size += count_multibyte_integer_size_for_value (record . uncompressed_size) ; } let padding_needed = (4 - (index_size % 4)) % 4 ; index_size += padding_needed ; index_size += 4 ; let backward_size = ((index_size / 4) - 1) as u32 ; let stream_flags = [0u8 , check_type as u8] ; let mut crc = CRC32 . digest () ; crc . update (& backward_size . to_le_bytes ()) ; crc . update (& stream_flags) ; writer . write_u32 (crc . finalize ()) ? ; writer . write_u32 (backward_size) ? ; writer . write_all (& stream_flags) ? ; writer . write_all (& XZ_FOOTER_MAGIC) ? ; Ok (()) }
    };
}

write_xz_stream_footer!()