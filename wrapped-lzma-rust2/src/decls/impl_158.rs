macro_rules! deps {
    () => {
        Result!();
        StreamHeader!();
        CheckType!();
        Read!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl StreamHeader { fn parse < R : Read > (reader : & mut R) -> crate :: Result < Self > { let mut magic = [0u8 ; 6] ; reader . read_exact (& mut magic) ? ; if magic != XZ_MAGIC { return Err (error_invalid_data ("invalid XZ magic bytes")) ; } Self :: parse_stream_header_flags_and_crc (reader) } pub (crate) fn parse_stream_header_flags_and_crc < R : Read > (reader : & mut R ,) -> crate :: Result < Self > { let mut flags = [0u8 ; 2] ; reader . read_exact (& mut flags) ? ; if flags [0] != 0 { return Err (error_invalid_data ("invalid XZ stream flags")) ; } let check_type = CheckType :: from_byte (flags [1]) ? ; let expected_crc = reader . read_u32 () ? ; if expected_crc != CRC32 . checksum (& flags) { return Err (error_invalid_data ("XZ stream header CRC32 mismatch")) ; } Ok (StreamHeader { check_type }) } }
    };
}

impl_158!();