macro_rules! read_u128 {
    () => {
        # [doc = " Read a u128 from the beginning of the given slice in native endian format."] # [doc = " If the slice has fewer than 16 bytes, then this panics."] pub (crate) fn read_u128 (slice : & [u8]) -> u128 { let bytes : [u8 ; 16] = slice [.. size_of :: < u128 > ()] . try_into () . unwrap () ; u128 :: from_ne_bytes (bytes) }
    };
}

read_u128!();