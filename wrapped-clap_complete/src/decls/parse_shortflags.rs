macro_rules! parse_shortflags {
    () => {
        # [doc = " Parse the short flags and find the first `takes_values` option."] fn parse_shortflags < 'c , 's > (cmd : & 'c clap :: Command , mut short : clap_lex :: ShortFlags < 's > ,) -> (String , Option < & 'c clap :: Arg > , clap_lex :: ShortFlags < 's >) { let takes_value_opt ; let mut leading_flags = String :: new () ; loop { match short . next_flag () { Some (Ok (opt)) => { leading_flags . push (opt) ; let opt = cmd . get_arguments () . find (| a | { let shorts = a . get_short_and_visible_aliases () ; let is_find = shorts . map (| v | { let mut iter = v . into_iter () ; let c = iter . find (| c | * c == opt) ; c . is_some () }) ; is_find . unwrap_or (false) }) ; if opt . map (| o | o . get_num_args () . expect ("built") . takes_values ()) . unwrap_or (false) { takes_value_opt = opt ; break ; } } Some (Err (_)) | None => { takes_value_opt = None ; break ; } } } (leading_flags , takes_value_opt , short) }
    };
}

parse_shortflags!();