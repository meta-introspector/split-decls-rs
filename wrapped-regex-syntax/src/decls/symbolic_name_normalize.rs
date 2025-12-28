macro_rules! symbolic_name_normalize {
    () => {
        # [doc = " Like symbolic_name_normalize_bytes, but operates on a string."] fn symbolic_name_normalize (x : & str) -> String { let mut tmp = x . as_bytes () . to_vec () ; let len = symbolic_name_normalize_bytes (& mut tmp) . len () ; tmp . truncate (len) ; String :: from_utf8 (tmp) . unwrap () }
    };
}

symbolic_name_normalize!()