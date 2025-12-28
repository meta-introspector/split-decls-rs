macro_rules! calculate_file_hash {
    () => {
        # [cfg (not (feature = "md5_enabled"))] pub fn calculate_file_hash (_content : & [u8]) -> String { "dummy_hash" . to_string () }
    };
}

calculate_file_hash!();