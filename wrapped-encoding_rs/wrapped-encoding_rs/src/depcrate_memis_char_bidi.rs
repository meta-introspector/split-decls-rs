// Generated macro for is_char_bidi (function)
macro_rules! Depcrate_memis_char_bidi {
() => {
// Module: crate::mem
// Provides: {"is_char_bidi"}
// Dependencies: {}
# [doc = " Checks whether a scalar value triggers right-to-left processing."] # [doc = ""] # [doc = " The check is done on a Unicode block basis without regard to assigned"] # [doc = " vs. unassigned code points in the block. Hebrew presentation forms in"] # [doc = " the Alphabetic Presentation Forms block are treated as if they formed"] # [doc = " a block on their own (i.e. it treated as right-to-left). Additionally,"] # [doc = " the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked"] # [doc = " for. Control characters that are technically bidi controls but do not"] # [doc = " cause right-to-left behavior without the presence of right-to-left"] # [doc = " characters or right-to-left controls are not checked for. As a special"] # [doc = " case, U+FEFF is excluded from Arabic Presentation Forms-B."] # [inline (always)] pub fn is_char_bidi (c : char) -> bool { let code_point = u32 :: from (c) ; if code_point < 0x0590 { return false ; } if in_range32 (code_point , 0x0900 , 0xFB1D) { if in_inclusive_range32 (code_point , 0x200F , 0x2067) { return code_point == 0x200F || code_point == 0x202B || code_point == 0x202E || code_point == 0x2067 ; } return false ; } if code_point > 0x1EFFF { return false ; } if in_range32 (code_point , 0x11000 , 0x1E800) { return false ; } if in_range32 (code_point , 0xFEFF , 0x10800) { return false ; } if in_range32 (code_point , 0xFE00 , 0xFE70) { return false ; } true }
};
}
