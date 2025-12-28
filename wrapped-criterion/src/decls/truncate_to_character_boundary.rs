macro_rules! truncate_to_character_boundary {
    () => {
        fn truncate_to_character_boundary (s : & mut String , max_len : usize) { let mut boundary = cmp :: min (max_len , s . len ()) ; while ! s . is_char_boundary (boundary) { boundary -= 1 ; } s . truncate (boundary) ; }
    };
}

truncate_to_character_boundary!()