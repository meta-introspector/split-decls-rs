macro_rules! to_u16_lanes {
    () => {
        # [inline (always)] pub fn to_u16_lanes (s : u8x16) -> u16x8 { u16x8 :: from_ne_bytes (s) }
    };
}

to_u16_lanes!();