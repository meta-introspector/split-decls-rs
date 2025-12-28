macro_rules! deps {
    () => {
        EncodeUtf8Error!();
    };
}

macro_rules! encode_utf8 {
    () => {
        deps!();
        # [doc = " Encode a char into buf using UTF-8."] # [doc = ""] # [doc = " On success, return the byte length of the encoding (1, 2, 3 or 4).<br>"] # [doc = " On error, return `EncodeUtf8Error` if the buffer was too short for the char."] # [doc = ""] # [doc = " Safety: `ptr` must be writable for `len` bytes."] # [inline] pub unsafe fn encode_utf8 (ch : char , ptr : * mut u8 , len : usize) -> Result < usize , EncodeUtf8Error > { let code = ch as u32 ; if code < MAX_ONE_B && len >= 1 { ptr . add (0) . write (code as u8) ; return Ok (1) ; } else if code < MAX_TWO_B && len >= 2 { ptr . add (0) . write ((code >> 6 & 0x1F) as u8 | TAG_TWO_B) ; ptr . add (1) . write ((code & 0x3F) as u8 | TAG_CONT) ; return Ok (2) ; } else if code < MAX_THREE_B && len >= 3 { ptr . add (0) . write ((code >> 12 & 0x0F) as u8 | TAG_THREE_B) ; ptr . add (1) . write ((code >> 6 & 0x3F) as u8 | TAG_CONT) ; ptr . add (2) . write ((code & 0x3F) as u8 | TAG_CONT) ; return Ok (3) ; } else if len >= 4 { ptr . add (0) . write ((code >> 18 & 0x07) as u8 | TAG_FOUR_B) ; ptr . add (1) . write ((code >> 12 & 0x3F) as u8 | TAG_CONT) ; ptr . add (2) . write ((code >> 6 & 0x3F) as u8 | TAG_CONT) ; ptr . add (3) . write ((code & 0x3F) as u8 | TAG_CONT) ; return Ok (4) ; } ; Err (EncodeUtf8Error) }
    };
}

encode_utf8!()