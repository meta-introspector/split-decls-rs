macro_rules! read_u32 {
    () => {
        # [doc = " Read a u32 from the beginning of the given slice in native endian format."] # [doc = " If the slice has fewer than 4 bytes, then this panics."] # [doc = ""] # [doc = " Marked as inline to speed up sparse searching which decodes integers from"] # [doc = " its automaton at search time."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn read_u32 (slice : & [u8]) -> u32 { let bytes : [u8 ; 4] = slice [.. size_of :: < u32 > ()] . try_into () . unwrap () ; u32 :: from_ne_bytes (bytes) }
    };
}

read_u32!()