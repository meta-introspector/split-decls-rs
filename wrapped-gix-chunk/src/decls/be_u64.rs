macro_rules! be_u64 {
    () => {
        fn be_u64 (data : & [u8]) -> u64 { u64 :: from_be_bytes (data [.. 8] . try_into () . unwrap ()) }
    };
}

be_u64!();