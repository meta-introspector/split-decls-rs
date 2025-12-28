macro_rules! deps {
    () => {
        ParseState!();
    };
}

macro_rules! parse_positional {
    () => {
        deps!();
        # [doc = " Parse the positional arguments. Return the new state and the new positional index."] fn parse_positional < 'a > (cmd : & clap :: Command , pos_index : usize , is_escaped : bool , state : ParseState < 'a > ,) -> (ParseState < 'a > , usize) { let pos_arg = cmd . get_positionals () . find (| p | p . get_index () == Some (pos_index)) ; let num_args = pos_arg . and_then (| a | a . get_num_args () . map (| r | r . max_values ())) . unwrap_or (1) ; let update_state_with_new_positional = | pos_index | -> (ParseState < 'a > , usize) { if num_args > 1 { (ParseState :: Pos ((pos_index , 1)) , pos_index) } else { if is_escaped { (ParseState :: Pos ((pos_index , 1)) , pos_index + 1) } else { (ParseState :: ValueDone , pos_index + 1) } } } ; match state { ParseState :: ValueDone => { update_state_with_new_positional (pos_index) } , ParseState :: Pos ((prev_pos_index , num_arg)) => { if prev_pos_index == pos_index { if num_arg + 1 < num_args { (ParseState :: Pos ((pos_index , num_arg + 1)) , pos_index) } else { if is_escaped { (ParseState :: Pos ((pos_index , 1)) , pos_index + 1) } else { (ParseState :: ValueDone , pos_index + 1) } } } else { update_state_with_new_positional (pos_index) } } ParseState :: Opt (..) => unreachable ! ("This branch won't be hit,
            because ParseState::Opt should not be seen as a positional argument and passed to this function.") , } }
    };
}

parse_positional!();