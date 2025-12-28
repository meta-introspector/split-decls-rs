macro_rules! is_utf8_continuation {
    () => {
        # [inline] fn is_utf8_continuation (b : u8) -> bool { matches ! (b , 0x80 ..= 0xbf) }
    };
}

is_utf8_continuation!();