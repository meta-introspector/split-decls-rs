// Generated macro for Atomicity (enum)
macro_rules! Depcrate_parser_stateAtomicity {
() => {
// Module: crate::parser_state
// Provides: {"Atomicity"}
// Dependencies: {}
# [doc = " The current atomicity of a [`ParserState`]."] # [doc = ""] # [doc = " [`ParserState`]: struct.ParserState.html"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Atomicity { # [doc = " prevents implicit whitespace: inside an atomic rule,"] # [doc = " the tilde ~ means \"immediately followed by\","] # [doc = " and repetition operators (asterisk * and plus sign +)"] # [doc = " have no implicit separation. In addition, all other rules"] # [doc = " called from an atomic rule are also treated as atomic."] # [doc = " (interior matching rules are silent)"] Atomic , # [doc = " The same as atomic, but inner tokens are produced as normal."] CompoundAtomic , # [doc = " implicit whitespace is enabled"] NonAtomic , }
};
}
