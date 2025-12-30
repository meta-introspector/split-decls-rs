// Generated macro for utf16_valid_up_to_alu (function)
macro_rules! Depcrate_memutf16_valid_up_to_alu {
() => {
// Module: crate::mem
// Provides: {"utf16_valid_up_to_alu"}
// Dependencies: {}
# [doc = " The second return value is true iff the last code unit of the slice was"] # [doc = " reached and turned out to be a low surrogate that is part of a valid pair."] # [allow (clippy :: collapsible_if)] # [inline (always)] fn utf16_valid_up_to_alu (buffer : & [u16]) -> (usize , bool) { let len = buffer . len () ; if len == 0 { return (0 , false) ; } let mut offset = 0usize ; loop { let unit = buffer [offset] ; let next = offset + 1 ; let unit_minus_surrogate_start = unit . wrapping_sub (0xD800) ; if unit_minus_surrogate_start > (0xDFFF - 0xD800) { offset = next ; if offset == len { return (offset , false) ; } continue ; } if unit_minus_surrogate_start <= (0xDBFF - 0xD800) { if next < len { let second = buffer [next] ; let second_minus_low_surrogate_start = second . wrapping_sub (0xDC00) ; if second_minus_low_surrogate_start <= (0xDFFF - 0xDC00) { offset = next + 1 ; if offset == len { return (offset , true) ; } continue ; } } } return (offset , false) ; } }
};
}
