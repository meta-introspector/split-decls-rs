// Generated macro for take_pat (function)
macro_rules! Depcrate_unnested_or_patternstake_pat {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"take_pat"}
// Dependencies: {}
# [doc = " Extract the pattern from the given one and replace it with `Wild`."] # [doc = " This is meant for temporarily swapping out the pattern for manipulation."] fn take_pat (from : & mut Pat) -> Pat { let dummy = Pat { id : DUMMY_NODE_ID , kind : Wild , span : DUMMY_SP , tokens : None , } ; mem :: replace (from , dummy) }
};
}
