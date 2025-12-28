macro_rules! deps {
    () => {
        Error!();
        PacketLineRef!();
        PacketLineOrWantedSize!();
    };
}

macro_rules! hex_prefix {
    () => {
        deps!();
        # [doc = " Decode the `four_bytes` packet line prefix provided in hexadecimal form and check it for validity."] pub fn hex_prefix (four_bytes : & [u8]) -> Result < PacketLineOrWantedSize < '_ > , Error > { debug_assert_eq ! (four_bytes . len () , 4 , "need four hex bytes") ; for (line_bytes , line_type) in & [(FLUSH_LINE , PacketLineRef :: Flush) , (DELIMITER_LINE , PacketLineRef :: Delimiter) , (RESPONSE_END_LINE , PacketLineRef :: ResponseEnd) ,] { if four_bytes == * line_bytes { return Ok (PacketLineOrWantedSize :: Line (* line_type)) ; } } let mut buf = [0u8 ; U16_HEX_BYTES / 2] ; faster_hex :: hex_decode (four_bytes , & mut buf) . map_err (| err | Error :: HexDecode { err : err . to_string () }) ? ; let wanted_bytes = u16 :: from_be_bytes (buf) ; if wanted_bytes == 3 { return Err (Error :: InvalidLineLength) ; } if wanted_bytes == 4 { return Err (Error :: DataIsEmpty) ; } debug_assert ! (wanted_bytes as usize > U16_HEX_BYTES , "by now there should be more wanted bytes than prefix bytes") ; Ok (PacketLineOrWantedSize :: Wanted (wanted_bytes - U16_HEX_BYTES as u16)) }
    };
}

hex_prefix!()