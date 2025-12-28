macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! parse_multibyte_integer {
    () => {
        deps!();
        # [doc = " Parse XZ multibyte integer (variable length encoding)."] fn parse_multibyte_integer (data : & [u8]) -> crate :: Result < u64 > { let mut result = 0u64 ; let mut shift = 0 ; for & byte in data { if shift >= 63 { return Err (error_invalid_data ("XZ multibyte integer too large")) ; } result |= ((byte & 0x7F) as u64) << shift ; shift += 7 ; if (byte & 0x80) == 0 { return Ok (result) ; } } Err (error_invalid_data ("incomplete XZ multibyte integer")) }
    };
}

parse_multibyte_integer!();