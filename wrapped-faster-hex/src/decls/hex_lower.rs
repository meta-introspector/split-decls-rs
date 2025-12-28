macro_rules! hex_lower {
    () => {
        # [inline] fn hex_lower (byte : u8) -> u8 { TABLE_LOWER [byte as usize] }
    };
}

hex_lower!();