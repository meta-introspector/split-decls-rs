macro_rules! is_char_boundary {
    () => {
        # [doc = " Test if a byte is the start of a UTF-8 character."] # [doc = " (extracted from `str::is_char_boundary`)"] # [inline] fn is_char_boundary (b : u8) -> bool { (b as i8) >= - 0x40 }
    };
}

is_char_boundary!();