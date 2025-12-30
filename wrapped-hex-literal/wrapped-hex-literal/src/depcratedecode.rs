// Generated macro for decode (function)
macro_rules! Depcratedecode {
() => {
// Module: crate
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decode hex strings into a byte array of pre-computed length."] # [doc = ""] # [doc = " This function is an implementation detail and SHOULD NOT be called directly!"] # [doc (hidden)] pub const fn decode < const LEN : usize > (strings : & [& [u8]]) -> Option < [u8 ; LEN] > { let mut string_pos = 0 ; let mut buf = [0u8 ; LEN] ; let mut buf_pos = 0 ; while string_pos < strings . len () { let mut pos = 0 ; let string = & strings [string_pos] ; string_pos += 1 ; while let Some ((byte , new_pos)) = next_byte (string , pos) { buf [buf_pos] = byte ; buf_pos += 1 ; pos = new_pos ; } } if LEN == buf_pos { Some (buf) } else { None } }
};
}
