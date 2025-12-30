// Generated macro for NameRefKind (enum)
macro_rules! Depcrate_contextNameRefKind {
() => {
// Module: crate::context
// Provides: {"NameRefKind"}
// Dependencies: {}
# [doc = " The kind of the NameRef we are completing."] # [derive (Debug)] pub (crate) enum NameRefKind < 'db > { Path (PathCompletionCtx < 'db >) , DotAccess (DotAccess < 'db >) , # [doc = " Position where we are only interested in keyword completions"] Keyword (ast :: Item) , # [doc = " The record expression this nameref is a field of and whether a dot precedes the completion identifier."] RecordExpr { dot_prefix : bool , expr : ast :: RecordExpr , } , Pattern (PatternContext) , ExternCrate , }
};
}
