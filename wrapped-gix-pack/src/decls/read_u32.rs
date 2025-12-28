macro_rules! read_u32 {
    () => {
        # [inline] fn read_u32 (b : & [u8]) -> u32 { u32 :: from_be_bytes (b . try_into () . unwrap ()) }
    };
}

read_u32!()