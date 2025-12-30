// Generated macro for span_contains_comment (function)
macro_rules! Depcratespan_contains_comment {
() => {
// Module: crate
// Provides: {"span_contains_comment"}
// Dependencies: {}
# [doc = " Checks whether a given span has any comment token"] # [doc = " This checks for all types of comment: line \"//\", block \"/**\", doc \"///\" \"//!\""] pub fn span_contains_comment (sm : & SourceMap , span : Span) -> bool { let Ok (snippet) = sm . span_to_snippet (span) else { return false ; } ; return tokenize (& snippet , FrontmatterAllowed :: No) . any (| token | { matches ! (token . kind , TokenKind :: BlockComment { .. } | TokenKind :: LineComment { .. }) }) ; }
};
}
