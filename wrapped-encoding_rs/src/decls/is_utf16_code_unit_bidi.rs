macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! is_utf16_code_unit_bidi {
    () => {
        deps!();
        # [doc = " Checks whether a UTF-16 code unit triggers right-to-left processing."] # [doc = ""] # [doc = " The check is done on a Unicode block basis without regard to assigned"] # [doc = " vs. unassigned code points in the block. Hebrew presentation forms in"] # [doc = " the Alphabetic Presentation Forms block are treated as if they formed"] # [doc = " a block on their own (i.e. it treated as right-to-left). Additionally,"] # [doc = " the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked"] # [doc = " for. Control characters that are technically bidi controls but do not"] # [doc = " cause right-to-left behavior without the presence of right-to-left"] # [doc = " characters or right-to-left controls are not checked for. As a special"] # [doc = " case, U+FEFF is excluded from Arabic Presentation Forms-B."] # [doc = ""] # [doc = " Since supplementary-plane right-to-left blocks are identifiable from the"] # [doc = " high surrogate without examining the low surrogate, this function returns"] # [doc = " `true` for such high surrogates making the function suitable for handling"] # [doc = " supplementary-plane text without decoding surrogate pairs to scalar"] # [doc = " values. Obviously, such high surrogates are then reported as right-to-left"] # [doc = " even if actually unpaired."] # [inline (always)] pub fn is_utf16_code_unit_bidi (u : u16) -> bool { if u < 0x0590 { return false ; } if in_range16 (u , 0x0900 , 0xD802) { if in_inclusive_range16 (u , 0x200F , 0x2067) { return u == 0x200F || u == 0x202B || u == 0x202E || u == 0x2067 ; } return false ; } if in_range16 (u , 0xD83C , 0xFB1D) { return false ; } if in_range16 (u , 0xD804 , 0xD83A) { return false ; } if u > 0xFEFE { return false ; } if in_range16 (u , 0xFE00 , 0xFE70) { return false ; } true }
    };
}

is_utf16_code_unit_bidi!();