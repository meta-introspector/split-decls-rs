macro_rules! read_u16 {
    () => {
        # [inline] fn read_u16 (data : & [u8]) -> Option < (u16 , & [u8]) > { data . split_at_checked (2) . map (| (num , data) | (u16 :: from_be_bytes (num . try_into () . unwrap ()) , data)) }
    };
}

read_u16!();