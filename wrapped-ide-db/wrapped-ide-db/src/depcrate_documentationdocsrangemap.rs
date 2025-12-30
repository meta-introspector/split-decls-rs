// Generated macro for DocsRangeMap (struct)
macro_rules! Depcrate_documentationDocsRangeMap {
() => {
// Module: crate::documentation
// Provides: {"DocsRangeMap"}
// Dependencies: {}
# [doc = " A struct to map text ranges from [`Documentation`] back to TextRanges in the syntax tree."] # [derive (Debug)] pub struct DocsRangeMap { source_map : AttrSourceMap , mapping : Vec < (TextRange , AttrId , TextRange) > , }
};
}
