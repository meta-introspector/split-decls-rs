// Generated macro for encode_utf8 (function)
macro_rules! Depcrate_charencode_utf8 {
() => {
// Module: crate::char
// Provides: {"encode_utf8"}
// Dependencies: {}
# [doc = " Encode a char into buf using UTF-8."] # [doc = ""] # [doc = " On success, return the byte length of the encoding (1, 2, 3 or 4).<br>"] # [doc = " On error, return `EncodeUtf8Error` if the buffer was too short for the char."] # [inline] pub fn encode_utf8 (ch : char , buf : & mut [u8]) -> Result < usize , EncodeUtf8Error > { let code = ch as u32 ; if code < MAX_ONE_B && buf . len () >= 1 { buf [0] = code as u8 ; return Ok (1) ; } else if code < MAX_TWO_B && buf . len () >= 2 { buf [0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B ; buf [1] = (code & 0x3F) as u8 | TAG_CONT ; return Ok (2) ; } else if code < MAX_THREE_B && buf . len () >= 3 { buf [0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B ; buf [1] = (code >> 6 & 0x3F) as u8 | TAG_CONT ; buf [2] = (code & 0x3F) as u8 | TAG_CONT ; return Ok (3) ; } else if buf . len () >= 4 { buf [0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B ; buf [1] = (code >> 12 & 0x3F) as u8 | TAG_CONT ; buf [2] = (code >> 6 & 0x3F) as u8 | TAG_CONT ; buf [3] = (code & 0x3F) as u8 | TAG_CONT ; return Ok (4) ; } ; Err (EncodeUtf8Error (())) }
};
}
