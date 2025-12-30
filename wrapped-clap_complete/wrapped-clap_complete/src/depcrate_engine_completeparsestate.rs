// Generated macro for ParseState (enum)
macro_rules! Depcrate_engine_completeParseState {
() => {
// Module: crate::engine::complete
// Provides: {"ParseState"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] enum ParseState < 'a > { # [doc = " Parsing a value done, there is no state to record."] ValueDone , # [doc = " Parsing a positional argument after `--`. `Pos(pos_index`, `takes_num_args`)"] Pos ((usize , usize)) , # [doc = " Parsing a optional flag argument"] Opt ((& 'a clap :: Arg , usize)) , }
};
}
