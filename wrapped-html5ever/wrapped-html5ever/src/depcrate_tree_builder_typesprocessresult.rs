// Generated macro for ProcessResult (enum)
macro_rules! Depcrate_tree_builder_typesProcessResult {
() => {
// Module: crate::tree_builder::types
// Provides: {"ProcessResult"}
// Dependencies: {}
pub enum ProcessResult { Done , DoneAckSelfClosing , SplitWhitespace (StrTendril) , Reprocess (InsertionMode , Token) , ReprocessForeign (Token) , }
};
}
