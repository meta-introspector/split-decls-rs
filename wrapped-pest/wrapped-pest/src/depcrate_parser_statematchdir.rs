// Generated macro for MatchDir (enum)
macro_rules! Depcrate_parser_stateMatchDir {
() => {
// Module: crate::parser_state
// Provides: {"MatchDir"}
// Dependencies: {}
# [doc = " Match direction for the stack. Used in `PEEK[a..b]`/`stack_match_peek_slice`."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum MatchDir { # [doc = " from the bottom to the top of the stack"] BottomToTop , # [doc = " from the top to the bottom of the stack"] TopToBottom , }
};
}
