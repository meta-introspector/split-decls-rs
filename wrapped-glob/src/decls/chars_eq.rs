macro_rules! chars_eq {
    () => {
        # [doc = " A helper function to determine if two chars are (possibly case-insensitively) equal."] fn chars_eq (a : char , b : char , case_sensitive : bool) -> bool { if cfg ! (windows) && path :: is_separator (a) && path :: is_separator (b) { true } else if ! case_sensitive && a . is_ascii () && b . is_ascii () { a . eq_ignore_ascii_case (& b) } else { a == b } }
    };
}

chars_eq!()