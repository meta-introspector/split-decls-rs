macro_rules! read_u16 {
    () => {
        # [doc = " Read a u16 from the beginning of the given slice in native endian format."] # [doc = " If the slice has fewer than 2 bytes, then this panics."] # [doc = ""] # [doc = " Marked as inline to speed up sparse searching which decodes integers from"] # [doc = " its automaton at search time."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn read_u16 (slice : & [u8]) -> u16 { let bytes : [u8 ; 2] = slice [.. size_of :: < u16 > ()] . try_into () . unwrap () ; u16 :: from_ne_bytes (bytes) }
    };
}

read_u16!();