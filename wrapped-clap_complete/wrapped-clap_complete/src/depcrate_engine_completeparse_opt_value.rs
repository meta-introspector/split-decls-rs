// Generated macro for parse_opt_value (function)
macro_rules! Depcrate_engine_completeparse_opt_value {
() => {
// Module: crate::engine::complete
// Provides: {"parse_opt_value"}
// Dependencies: {}
# [doc = " Parse optional flag argument. Return new state"] fn parse_opt_value (opt : & clap :: Arg , count : usize) -> ParseState < '_ > { let range = opt . get_num_args () . expect ("built") ; let max = range . max_values () ; if count < max { ParseState :: Opt ((opt , count + 1)) } else { ParseState :: ValueDone } }
};
}
