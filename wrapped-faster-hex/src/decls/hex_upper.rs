macro_rules! hex_upper {
    () => {
        # [inline] fn hex_upper (byte : u8) -> u8 { TABLE_UPPER [byte as usize] }
    };
}

hex_upper!();