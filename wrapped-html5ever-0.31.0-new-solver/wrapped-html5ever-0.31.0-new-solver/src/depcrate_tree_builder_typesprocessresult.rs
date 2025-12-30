// Generated macro for ProcessResult (enum)
macro_rules! Depcrate_tree_builder_typesProcessResult {
() => {
// Module: crate::tree_builder::types
// Provides: {"ProcessResult"}
// Dependencies: {}
pub (crate) enum ProcessResult < Handle > { Done , DoneAckSelfClosing , SplitWhitespace (StrTendril) , Reprocess (InsertionMode , Token) , # [allow (dead_code)] ReprocessForeign (Token) , Script (Handle) , ToPlaintext , ToRawData (RawKind) , }
};
}
