// Generated macro for decode (function)
macro_rules! Depcrate_utf8decode {
() => {
// Module: crate::utf8
// Provides: {"decode"}
// Dependencies: {}
# [doc = " UTF-8 decode a single Unicode scalar value from the beginning of a slice."] # [doc = ""] # [doc = " When successful, the corresponding Unicode scalar value is returned along"] # [doc = " with the number of bytes it was encoded with. The number of bytes consumed"] # [doc = " for a successful decode is always between 1 and 4, inclusive."] # [doc = ""] # [doc = " When unsuccessful, `None` is returned along with the number of bytes that"] # [doc = " make up a maximal prefix of a valid UTF-8 code unit sequence. In this case,"] # [doc = " the number of bytes consumed is always between 0 and 3, inclusive, where"] # [doc = " 0 is only returned when `slice` is empty."] pub (crate) fn decode < B : AsRef < [u8] > > (slice : B) -> (Option < char > , usize) { let slice = slice . as_ref () ; match slice . get (0) { None => return (None , 0) , Some (& b) if b <= 0x7F => return (Some (b as char) , 1) , _ => { } } let (mut state , mut cp , mut i) = (ACCEPT , 0 , 0) ; while i < slice . len () { decode_step (& mut state , & mut cp , slice [i]) ; i += 1 ; if state == ACCEPT { let ch = char :: from_u32 (cp) . unwrap () ; return (Some (ch) , i) ; } else if state == REJECT { return (None , core :: cmp :: max (1 , i . saturating_sub (1))) ; } } (None , i) }
};
}
