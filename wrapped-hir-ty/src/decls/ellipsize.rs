macro_rules! ellipsize {
    () => {
        fn ellipsize (mut text : String , max_len : usize) -> String { if text . len () <= max_len { return text ; } let ellipsis = "..." ; let e_len = ellipsis . len () ; let mut prefix_len = (max_len - e_len) / 2 ; while ! text . is_char_boundary (prefix_len) { prefix_len += 1 ; } let mut suffix_len = max_len - e_len - prefix_len ; while ! text . is_char_boundary (text . len () - suffix_len) { suffix_len += 1 ; } text . replace_range (prefix_len .. text . len () - suffix_len , ellipsis) ; text }
    };
}

ellipsize!()