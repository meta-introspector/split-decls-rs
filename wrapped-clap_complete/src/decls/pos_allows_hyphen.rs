macro_rules! pos_allows_hyphen {
    () => {
        fn pos_allows_hyphen (cmd : & clap :: Command , pos_index : usize) -> bool { cmd . get_positionals () . find (| a | a . get_index () == Some (pos_index)) . map (| p | p . is_allow_hyphen_values_set ()) . unwrap_or (false) }
    };
}

pos_allows_hyphen!();