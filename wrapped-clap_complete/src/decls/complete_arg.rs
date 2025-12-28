macro_rules! deps {
    () => {
        CompletionCandidate!();
        ParseState!();
    };
}

macro_rules! complete_arg {
    () => {
        deps!();
        fn complete_arg (arg : & clap_lex :: ParsedArg < '_ > , cmd : & clap :: Command , current_dir : Option < & std :: path :: Path > , pos_index : usize , is_escaped : bool , state : ParseState < '_ > ,) -> Result < Vec < CompletionCandidate > , std :: io :: Error > { debug ! ("complete_arg: arg={:?}, cmd={:?}, current_dir={:?}, pos_index={:?}, state={:?}" , arg , cmd . get_name () , current_dir , pos_index , state) ; let mut completions = Vec :: < CompletionCandidate > :: new () ; match state { ParseState :: ValueDone => { if let Ok (value) = arg . to_value () { completions . extend (complete_subcommand (value , cmd)) ; } if let Some (positional) = cmd . get_positionals () . find (| p | p . get_index () == Some (pos_index)) { completions . extend (complete_arg_value (arg . to_value () , positional , current_dir)) ; } if ! is_escaped { completions . extend (complete_option (arg , cmd , current_dir)) ; } } ParseState :: Pos ((_ , num_arg)) => { if let Some (positional) = cmd . get_positionals () . find (| p | p . get_index () == Some (pos_index)) { completions . extend (complete_arg_value (arg . to_value () , positional , current_dir)) ; if positional . get_num_args () . is_some_and (| num_args | num_arg >= num_args . min_values ()) { completions . extend (complete_option (arg , cmd , current_dir)) ; } } } ParseState :: Opt ((opt , count)) => { completions . extend (complete_arg_value (arg . to_value () , opt , current_dir)) ; let min = opt . get_num_args () . map (| r | r . min_values ()) . unwrap_or (0) ; if count > min { completions . extend (complete_arg (arg , cmd , current_dir , pos_index , is_escaped , ParseState :: ValueDone ,) ?) ; } } } if completions . iter () . any (| a | ! a . is_hide_set ()) { completions . retain (| a | ! a . is_hide_set ()) ; } let mut seen_ids = std :: collections :: HashSet :: new () ; completions . retain (move | a | { if let Some (id) = a . get_id () . cloned () { seen_ids . insert (id) } else { true } }) ; let mut tags = Vec :: new () ; for candidate in & completions { let tag = candidate . get_tag () . cloned () ; if ! tags . contains (& tag) { tags . push (tag) ; } } completions . sort_by_key (| c | { (tags . iter () . position (| t | c . get_tag () == t . as_ref ()) , c . get_display_order () ,) }) ; Ok (completions) }
    };
}

complete_arg!();