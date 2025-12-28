macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! is_utf16_bidi {
    () => {
        deps!();
        # [doc = " Checks whether a UTF-16 buffer contains code points that trigger"] # [doc = " right-to-left processing."] # [doc = ""] # [doc = " The check is done on a Unicode block basis without regard to assigned"] # [doc = " vs. unassigned code points in the block. Hebrew presentation forms in"] # [doc = " the Alphabetic Presentation Forms block are treated as if they formed"] # [doc = " a block on their own (i.e. it treated as right-to-left). Additionally,"] # [doc = " the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked"] # [doc = " for. Control characters that are technically bidi controls but do not"] # [doc = " cause right-to-left behavior without the presence of right-to-left"] # [doc = " characters or right-to-left controls are not checked for. As a special"] # [doc = " case, U+FEFF is excluded from Arabic Presentation Forms-B."] # [doc = ""] # [doc = " Returns `true` if the input contains an RTL character or an unpaired"] # [doc = " high surrogate that could be the high half of an RTL character."] # [doc = " Returns `false` if the input contains neither RTL characters nor"] # [doc = " unpaired high surrogates that could be higher halves of RTL characters."] pub fn is_utf16_bidi (buffer : & [u16]) -> bool { is_utf16_bidi_impl (buffer) }
    };
}

is_utf16_bidi!();