macro_rules! is_string_continue_skipable_whitespace {
    () => {
        # [doc = " Checks whether the character is skipped after a string continue start"] # [doc = " (unescaped backlash followed by `\\n`)."] fn is_string_continue_skipable_whitespace (b : u8) -> bool { b == b' ' || b == b'\t' || b == b'\n' }
    };
}

is_string_continue_skipable_whitespace!()