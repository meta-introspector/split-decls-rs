macro_rules! bytes2path {
    () => {
        # [cfg (windows)] pub fn bytes2path (b : & [u8]) -> & Path { use std :: str ; Path :: new (str :: from_utf8 (b) . unwrap ()) }
    };
}

bytes2path!();