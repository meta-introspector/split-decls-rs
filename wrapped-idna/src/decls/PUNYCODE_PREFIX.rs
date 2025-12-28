macro_rules! PUNYCODE_PREFIX {
    () => {
        const PUNYCODE_PREFIX : u32 = ((b'-' as u32) << 24) | ((b'-' as u32) << 16) | ((b'N' as u32) << 8) | b'X' as u32 ;
    };
}

PUNYCODE_PREFIX!()