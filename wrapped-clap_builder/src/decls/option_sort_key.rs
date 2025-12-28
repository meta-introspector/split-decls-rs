macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! option_sort_key {
    () => {
        deps!();
        fn option_sort_key (arg : & Arg) -> (usize , String) { let key = if let Some (x) = arg . get_short () { let mut s = x . to_ascii_lowercase () . to_string () ; s . push (if x . is_ascii_lowercase () { '0' } else { '1' }) ; s } else if let Some (x) = arg . get_long () { x . to_string () } else { let mut s = '{' . to_string () ; s . push_str (arg . get_id () . as_str ()) ; s } ; (arg . get_display_order () , key) }
    };
}

option_sort_key!()