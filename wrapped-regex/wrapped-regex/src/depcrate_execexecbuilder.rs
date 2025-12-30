// Generated macro for ExecBuilder (struct)
macro_rules! Depcrate_execExecBuilder {
() => {
// Module: crate::exec
// Provides: {"ExecBuilder"}
// Dependencies: {}
# [doc = " Facilitates the construction of an executor by exposing various knobs"] # [doc = " to control how a regex is executed and what kinds of resources it's"] # [doc = " permitted to use."] pub struct ExecBuilder { options : RegexOptions , match_type : Option < MatchType > , bytes : bool , only_utf8 : bool , }
};
}
