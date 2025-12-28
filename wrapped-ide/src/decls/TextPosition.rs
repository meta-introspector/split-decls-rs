macro_rules! TextPosition {
    () => {
        struct TextPosition { offset : TextSize , line : u32 , col : u32 , }
    };
}

TextPosition!()