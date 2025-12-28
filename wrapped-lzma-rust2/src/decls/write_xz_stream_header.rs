macro_rules! deps {
    () => {
        CheckType!();
        Result!();
        Write!();
    };
}

macro_rules! write_xz_stream_header {
    () => {
        deps!();
        # [cfg (feature = "encoder")] fn write_xz_stream_header < W : Write > (writer : & mut W , check_type : CheckType) -> crate :: Result < () > { writer . write_all (& XZ_MAGIC) ? ; let stream_flags = [0u8 , check_type as u8] ; writer . write_all (& stream_flags) ? ; let crc = CRC32 . checksum (& stream_flags) ; writer . write_u32 (crc) ? ; Ok (()) }
    };
}

write_xz_stream_header!()