macro_rules! THIN_LTO_KEYS_INCR_COMP_FILE_NAME {
    () => {
        # [doc = " We keep track of the computed LTO cache keys from the previous"] # [doc = " session to determine which CGUs we can reuse."] const THIN_LTO_KEYS_INCR_COMP_FILE_NAME : & str = "thin-lto-past-keys.bin" ;
    };
}

THIN_LTO_KEYS_INCR_COMP_FILE_NAME!()