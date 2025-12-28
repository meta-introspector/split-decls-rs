macro_rules! coder_get_dict_size {
    () => {
        pub (crate) fn coder_get_dict_size (len : usize) -> usize { if len < DIST_STATES + MATCH_LEN_MIN { len - MATCH_LEN_MIN } else { DIST_STATES - 1 } }
    };
}

coder_get_dict_size!();