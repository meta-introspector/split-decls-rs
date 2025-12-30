// Generated macro for NextParserState (enum)
macro_rules! Depcrate_tree_builder_interfaceNextParserState {
() => {
// Module: crate::tree_builder::interface
// Provides: {"NextParserState"}
// Dependencies: {}
# [doc = " Whether to interrupt further parsing of the current input until"] # [doc = " the next explicit resumption of the tokenizer, or continue without"] # [doc = " any interruption."] # [derive (PartialEq , Eq , Copy , Clone , Hash , Debug)] pub enum NextParserState { Suspend , Continue , }
};
}
