// Generated macro for MatchGroup (struct)
macro_rules! Depcrate_match_group_typesMatchGroup {
() => {
// Module: crate::match_group::types
// Provides: {"MatchGroup"}
// Dependencies: {}
# [doc = " A match group is able to match a list of ref specs in order while handling negation, conflicts and one to many mappings."] # [derive (Default , Debug , Clone)] pub struct MatchGroup < 'a > { # [doc = " The specs that take part in item matching."] pub specs : Vec < RefSpecRef < 'a > > , }
};
}
