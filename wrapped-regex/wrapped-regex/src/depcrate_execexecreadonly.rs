// Generated macro for ExecReadOnly (struct)
macro_rules! Depcrate_execExecReadOnly {
() => {
// Module: crate::exec
// Provides: {"ExecReadOnly"}
// Dependencies: {}
# [doc = " ExecReadOnly comprises all read only state for a regex. Namely, all such"] # [doc = " state is determined at compile time and never changes during search."] # [derive (Debug)] struct ExecReadOnly { # [doc = " The original regular expressions given by the caller to compile."] res : Vec < String > , # [doc = " A compiled program that is used in the NFA simulation and backtracking."] # [doc = " It can be byte-based or Unicode codepoint based."] # [doc = ""] # [doc = " N.B. It is not possibly to make this byte-based from the public API."] # [doc = " It is only used for testing byte based programs in the NFA simulations."] nfa : Program , # [doc = " A compiled byte based program for DFA execution. This is only used"] # [doc = " if a DFA can be executed. (Currently, only word boundary assertions are"] # [doc = " not supported.) Note that this program contains an embedded `.*?`"] # [doc = " preceding the first capture group, unless the regex is anchored at the"] # [doc = " beginning."] dfa : Program , # [doc = " The same as above, except the program is reversed (and there is no"] # [doc = " preceding `.*?`). This is used by the DFA to find the starting location"] # [doc = " of matches."] dfa_reverse : Program , # [doc = " A set of suffix literals extracted from the regex."] # [doc = ""] # [doc = " Prefix literals are stored on the `Program`, since they are used inside"] # [doc = " the matching engines."] suffixes : LiteralSearcher , # [doc = " match_type encodes as much upfront knowledge about how we're going to"] # [doc = " execute a search as possible."] match_type : MatchType , }
};
}
