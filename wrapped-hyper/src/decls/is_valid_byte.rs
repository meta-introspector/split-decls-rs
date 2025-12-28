macro_rules! is_valid_byte {
    () => {
        const fn is_valid_byte (b : u8) -> bool { const fn is_vchar (b : u8) -> bool { 0x21 <= b && b <= 0x7E } # [allow (unused_comparisons , clippy :: absurd_extreme_comparisons)] const fn is_obs_text (b : u8) -> bool { 0x80 <= b && b <= 0xFF } b == b'\t' || b == b' ' || is_vchar (b) || is_obs_text (b) }
    };
}

is_valid_byte!();