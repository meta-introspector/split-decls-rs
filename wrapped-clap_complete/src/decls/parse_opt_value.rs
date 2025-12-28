macro_rules! deps {
    () => {
        ParseState!();
    };
}

macro_rules! parse_opt_value {
    () => {
        deps!();
        # [doc = " Parse optional flag argument. Return new state"] fn parse_opt_value (opt : & clap :: Arg , count : usize) -> ParseState < '_ > { let range = opt . get_num_args () . expect ("built") ; let max = range . max_values () ; if count < max { ParseState :: Opt ((opt , count + 1)) } else { ParseState :: ValueDone } }
    };
}

parse_opt_value!()