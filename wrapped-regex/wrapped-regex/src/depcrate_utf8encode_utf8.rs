// Generated macro for encode_utf8 (function)
macro_rules! Depcrate_utf8encode_utf8 {
() => {
// Module: crate::utf8
// Provides: {"encode_utf8"}
// Dependencies: {}
# [doc = " Encode the given Unicode character to `dst` as a single UTF-8 sequence."] # [doc = ""] # [doc = " If `dst` is not long enough, then `None` is returned. Otherwise, the number"] # [doc = " of bytes written is returned."] # [allow (dead_code)] # [inline] pub fn encode_utf8 (character : char , dst : & mut [u8]) -> Option < usize > { let code = character as u32 ; if code <= 0x7F && ! dst . is_empty () { dst [0] = code as u8 ; Some (1) } else if code <= 0x7FF && dst . len () >= 2 { dst [0] = (code >> 6 & 0x1F) as u8 | TAG_TWO ; dst [1] = (code & 0x3F) as u8 | TAG_CONT ; Some (2) } else if code <= 0xFFFF && dst . len () >= 3 { dst [0] = (code >> 12 & 0x0F) as u8 | TAG_THREE ; dst [1] = (code >> 6 & 0x3F) as u8 | TAG_CONT ; dst [2] = (code & 0x3F) as u8 | TAG_CONT ; Some (3) } else if dst . len () >= 4 { dst [0] = (code >> 18 & 0x07) as u8 | TAG_FOUR ; dst [1] = (code >> 12 & 0x3F) as u8 | TAG_CONT ; dst [2] = (code >> 6 & 0x3F) as u8 | TAG_CONT ; dst [3] = (code & 0x3F) as u8 | TAG_CONT ; Some (4) } else { None } }
};
}
