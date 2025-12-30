// Generated macro for MatchType (enum)
macro_rules! Depcrate_execMatchType {
() => {
// Module: crate::exec
// Provides: {"MatchType"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] enum MatchType { # [doc = " A single or multiple literal search. This is only used when the regex"] # [doc = " can be decomposed into unambiguous literal search."] Literal (MatchLiteralType) , # [doc = " A normal DFA search."] Dfa , # [doc = " A reverse DFA search starting from the end of a haystack."] DfaAnchoredReverse , # [doc = " A reverse DFA search with suffix literal scanning."] DfaSuffix , # [doc = " Use the DFA on two or more regular expressions."] DfaMany , # [doc = " An NFA variant."] Nfa (MatchNfaType) , # [doc = " No match is ever possible, so don't ever try to search."] Nothing , }
};
}
