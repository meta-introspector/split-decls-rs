macro_rules! decode {
    () => {
        pub (crate) mod decode { # [inline] pub (crate) fn u32 (data : & [u8]) -> Option < (u32 , & [u8]) > { data . split_at_checked (4) . map (| (num , data) | (u32 :: from_be_bytes (num . try_into () . unwrap ()) , data)) } }
    };
}

decode!()