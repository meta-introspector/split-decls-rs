// Generated macro for Config (struct)
macro_rules! Depcrate_nfa_thompson_compilerConfig {
() => {
// Module: crate::nfa::thompson::compiler
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration used for a Thompson NFA compiler."] # [derive (Clone , Debug , Default)] pub struct Config { utf8 : Option < bool > , reverse : Option < bool > , nfa_size_limit : Option < Option < usize > > , shrink : Option < bool > , which_captures : Option < WhichCaptures > , look_matcher : Option < LookMatcher > , # [cfg (test)] unanchored_prefix : Option < bool > , }
};
}
