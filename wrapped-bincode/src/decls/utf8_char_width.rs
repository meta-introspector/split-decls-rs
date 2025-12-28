macro_rules! utf8_char_width {
    () => {
        const fn utf8_char_width (b : u8) -> usize { UTF8_CHAR_WIDTH [b as usize] as usize }
    };
}

utf8_char_width!();