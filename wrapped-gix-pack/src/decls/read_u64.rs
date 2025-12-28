macro_rules! read_u64 {
    () => {
        # [inline] fn read_u64 (b : & [u8]) -> u64 { u64 :: from_be_bytes (b . try_into () . unwrap ()) }
    };
}

read_u64!();