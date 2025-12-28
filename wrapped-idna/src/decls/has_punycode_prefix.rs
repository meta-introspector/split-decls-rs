macro_rules! has_punycode_prefix {
    () => {
        # [inline (always)] fn has_punycode_prefix (slice : & [u8]) -> bool { if slice . len () < 4 { return false ; } let a = slice [0] ; let b = slice [1] ; let c = slice [2] ; let d = slice [3] ; let u = (u32 :: from (d) << 24) | (u32 :: from (c) << 16) | (u32 :: from (b) << 8) | u32 :: from (a) ; (u & PUNYCODE_PREFIX_MASK) == PUNYCODE_PREFIX }
    };
}

has_punycode_prefix!()