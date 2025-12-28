macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! convert_path {
    () => {
        deps!();
        # [cfg (not (unix))] fn convert_path (bytes : & [u8]) -> Result < PathBuf > { let s = std :: str :: from_utf8 (bytes) ? ; Ok (PathBuf :: from (s)) }
    };
}

convert_path!();