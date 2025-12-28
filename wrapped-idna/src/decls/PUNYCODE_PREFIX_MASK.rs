macro_rules! PUNYCODE_PREFIX_MASK {
    () => {
        const PUNYCODE_PREFIX_MASK : u32 = (0xFF << 24) | (0xFF << 16) | (0xDF << 8) | 0xDF ;
    };
}

PUNYCODE_PREFIX_MASK!();